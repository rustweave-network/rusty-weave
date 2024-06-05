use super::error::Result;
use core::fmt::Debug;
use kaspa_grpc_core::{
    ops::RustweavedPayloadOps,
    protowire::{RustweavedRequest, RustweavedResponse},
};
use std::{sync::Arc, time::Duration};
use tokio::sync::oneshot;

pub(crate) mod id;
pub(crate) mod matcher;
pub(crate) mod queue;

pub(crate) trait Resolver: Send + Sync + Debug {
    fn register_request(&self, op: RustweavedPayloadOps, request: &RustweavedRequest) -> RustweavedResponseReceiver;
    fn handle_response(&self, response: RustweavedResponse);
    fn remove_expired_requests(&self, timeout: Duration);
}

pub(crate) type DynResolver = Arc<dyn Resolver>;

pub(crate) type RustweavedResponseSender = oneshot::Sender<Result<RustweavedResponse>>;
pub(crate) type RustweavedResponseReceiver = oneshot::Receiver<Result<RustweavedResponse>>;
