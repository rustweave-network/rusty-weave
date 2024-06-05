use super::method::{DropFn, Method, MethodTrait, RoutingPolicy};
use crate::{
    connection::Connection,
    connection_handler::ServerContext,
    error::{GrpcServerError, GrpcServerResult},
};
use kaspa_grpc_core::{
    ops::RustweavedPayloadOps,
    protowire::{RustweavedRequest, RustweavedResponse},
};
use std::fmt::Debug;
use std::{collections::HashMap, sync::Arc};

pub type RustweavedMethod = Method<ServerContext, Connection, RustweavedRequest, RustweavedResponse>;
pub type DynRustweavedMethod = Arc<dyn MethodTrait<ServerContext, Connection, RustweavedRequest, RustweavedResponse>>;
pub type RustweavedDropFn = DropFn<RustweavedRequest, RustweavedResponse>;
pub type RustweavedRoutingPolicy = RoutingPolicy<RustweavedRequest, RustweavedResponse>;

/// An interface providing methods implementations and a fallback "not implemented" method
/// actually returning a message with a "not implemented" error.
///
/// The interface can provide a method clone for every [`RustweavedPayloadOps`] variant for later
/// processing of related requests.
///
/// It is also possible to directly let the interface itself process a request by invoking
/// the `call()` method.
pub struct Interface {
    server_ctx: ServerContext,
    methods: HashMap<RustweavedPayloadOps, DynRustweavedMethod>,
    method_not_implemented: DynRustweavedMethod,
}

impl Interface {
    pub fn new(server_ctx: ServerContext) -> Self {
        let method_not_implemented = Arc::new(Method::new(|_, _, rustweaved_request: RustweavedRequest| {
            Box::pin(async move {
                match rustweaved_request.payload {
                    Some(ref request) => Ok(RustweavedResponse {
                        id: rustweaved_request.id,
                        payload: Some(RustweavedPayloadOps::from(request).to_error_response(GrpcServerError::MethodNotImplemented.into())),
                    }),
                    None => Err(GrpcServerError::InvalidRequestPayload),
                }
            })
        }));
        Self { server_ctx, methods: Default::default(), method_not_implemented }
    }

    pub fn method(&mut self, op: RustweavedPayloadOps, method: RustweavedMethod) {
        let method: DynRustweavedMethod = Arc::new(method);
        if self.methods.insert(op, method).is_some() {
            panic!("RPC method {op:?} is declared multiple times")
        }
    }

    pub fn replace_method(&mut self, op: RustweavedPayloadOps, method: RustweavedMethod) {
        let method: DynRustweavedMethod = Arc::new(method);
        let _ = self.methods.insert(op, method);
    }

    pub fn set_method_properties(
        &mut self,
        op: RustweavedPayloadOps,
        tasks: usize,
        queue_size: usize,
        routing_policy: RustweavedRoutingPolicy,
    ) {
        self.methods.entry(op).and_modify(|x| {
            let method: Method<ServerContext, Connection, RustweavedRequest, RustweavedResponse> =
                Method::with_properties(x.method_fn(), tasks, queue_size, routing_policy);
            let method: Arc<dyn MethodTrait<ServerContext, Connection, RustweavedRequest, RustweavedResponse>> = Arc::new(method);
            *x = method;
        });
    }

    pub async fn call(
        &self,
        op: &RustweavedPayloadOps,
        connection: Connection,
        request: RustweavedRequest,
    ) -> GrpcServerResult<RustweavedResponse> {
        self.methods.get(op).unwrap_or(&self.method_not_implemented).call(self.server_ctx.clone(), connection, request).await
    }

    pub fn get_method(&self, op: &RustweavedPayloadOps) -> DynRustweavedMethod {
        self.methods.get(op).unwrap_or(&self.method_not_implemented).clone()
    }
}

impl Debug for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interface").finish()
    }
}
