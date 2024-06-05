use crate::protowire::{rustweaved_request, RustweavedRequest, RustweavedResponse};

impl From<rustweaved_request::Payload> for RustweavedRequest {
    fn from(item: rustweaved_request::Payload) -> Self {
        RustweavedRequest { id: 0, payload: Some(item) }
    }
}

impl AsRef<RustweavedRequest> for RustweavedRequest {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<RustweavedResponse> for RustweavedResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

pub mod rustweaved_request_convert {
    use crate::protowire::*;
    use kaspa_rpc_core::{RpcError, RpcResult};

    impl_into_rustweaved_request!(Shutdown);
    impl_into_rustweaved_request!(SubmitBlock);
    impl_into_rustweaved_request!(GetBlockTemplate);
    impl_into_rustweaved_request!(GetBlock);
    impl_into_rustweaved_request!(GetInfo);

    impl_into_rustweaved_request!(GetCurrentNetwork);
    impl_into_rustweaved_request!(GetPeerAddresses);
    impl_into_rustweaved_request!(GetSink);
    impl_into_rustweaved_request!(GetMempoolEntry);
    impl_into_rustweaved_request!(GetMempoolEntries);
    impl_into_rustweaved_request!(GetConnectedPeerInfo);
    impl_into_rustweaved_request!(AddPeer);
    impl_into_rustweaved_request!(SubmitTransaction);
    impl_into_rustweaved_request!(GetSubnetwork);
    impl_into_rustweaved_request!(GetVirtualChainFromBlock);
    impl_into_rustweaved_request!(GetBlocks);
    impl_into_rustweaved_request!(GetBlockCount);
    impl_into_rustweaved_request!(GetBlockDagInfo);
    impl_into_rustweaved_request!(ResolveFinalityConflict);
    impl_into_rustweaved_request!(GetHeaders);
    impl_into_rustweaved_request!(GetUtxosByAddresses);
    impl_into_rustweaved_request!(GetBalanceByAddress);
    impl_into_rustweaved_request!(GetBalancesByAddresses);
    impl_into_rustweaved_request!(GetSinkBlueScore);
    impl_into_rustweaved_request!(Ban);
    impl_into_rustweaved_request!(Unban);
    impl_into_rustweaved_request!(EstimateNetworkHashesPerSecond);
    impl_into_rustweaved_request!(GetMempoolEntriesByAddresses);
    impl_into_rustweaved_request!(GetCoinSupply);
    impl_into_rustweaved_request!(Ping);
    impl_into_rustweaved_request!(GetMetrics);
    impl_into_rustweaved_request!(GetServerInfo);
    impl_into_rustweaved_request!(GetSyncStatus);
    impl_into_rustweaved_request!(GetDaaScoreTimestampEstimate);

    impl_into_rustweaved_request!(NotifyBlockAdded);
    impl_into_rustweaved_request!(NotifyNewBlockTemplate);
    impl_into_rustweaved_request!(NotifyUtxosChanged);
    impl_into_rustweaved_request!(NotifyPruningPointUtxoSetOverride);
    impl_into_rustweaved_request!(NotifyFinalityConflict);
    impl_into_rustweaved_request!(NotifyVirtualDaaScoreChanged);
    impl_into_rustweaved_request!(NotifyVirtualChainChanged);
    impl_into_rustweaved_request!(NotifySinkBlueScoreChanged);

    macro_rules! impl_into_rustweaved_request {
        ($name:tt) => {
            paste::paste! {
                impl_into_rustweaved_request_ex!(kaspa_rpc_core::[<$name Request>],[<$name RequestMessage>],[<$name Request>]);
            }
        };
    }

    use impl_into_rustweaved_request;

    macro_rules! impl_into_rustweaved_request_ex {
        // ($($core_struct:ident)::+, $($protowire_struct:ident)::+, $($variant:ident)::+) => {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<&$core_struct> for rustweaved_request::Payload {
                fn from(item: &$core_struct) -> Self {
                    Self::$variant(item.into())
                }
            }

            impl From<&$core_struct> for RustweavedRequest {
                fn from(item: &$core_struct) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl From<$core_struct> for rustweaved_request::Payload {
                fn from(item: $core_struct) -> Self {
                    Self::$variant((&item).into())
                }
            }

            impl From<$core_struct> for RustweavedRequest {
                fn from(item: $core_struct) -> Self {
                    Self { id: 0, payload: Some((&item).into()) }
                }
            }

            // ----------------------------------------------------------------------------
            // protowire to rpc_core
            // ----------------------------------------------------------------------------

            impl TryFrom<&rustweaved_request::Payload> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &rustweaved_request::Payload) -> RpcResult<Self> {
                    if let rustweaved_request::Payload::$variant(request) = item {
                        request.try_into()
                    } else {
                        Err(RpcError::MissingRpcFieldError("Payload".to_string(), stringify!($variant).to_string()))
                    }
                }
            }

            impl TryFrom<&RustweavedRequest> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &RustweavedRequest) -> RpcResult<Self> {
                    item.payload
                        .as_ref()
                        .ok_or(RpcError::MissingRpcFieldError("RustweaveRequest".to_string(), "Payload".to_string()))?
                        .try_into()
                }
            }

            impl From<$protowire_struct> for RustweavedRequest {
                fn from(item: $protowire_struct) -> Self {
                    Self { id: 0, payload: Some(rustweaved_request::Payload::$variant(item)) }
                }
            }

            impl From<$protowire_struct> for rustweaved_request::Payload {
                fn from(item: $protowire_struct) -> Self {
                    rustweaved_request::Payload::$variant(item)
                }
            }
        };
    }
    use impl_into_rustweaved_request_ex;
}

pub mod rustweaved_response_convert {
    use crate::protowire::*;
    use kaspa_rpc_core::{RpcError, RpcResult};

    impl_into_rustweaved_response!(Shutdown);
    impl_into_rustweaved_response!(SubmitBlock);
    impl_into_rustweaved_response!(GetBlockTemplate);
    impl_into_rustweaved_response!(GetBlock);
    impl_into_rustweaved_response!(GetInfo);
    impl_into_rustweaved_response!(GetCurrentNetwork);

    impl_into_rustweaved_response!(GetPeerAddresses);
    impl_into_rustweaved_response!(GetSink);
    impl_into_rustweaved_response!(GetMempoolEntry);
    impl_into_rustweaved_response!(GetMempoolEntries);
    impl_into_rustweaved_response!(GetConnectedPeerInfo);
    impl_into_rustweaved_response!(AddPeer);
    impl_into_rustweaved_response!(SubmitTransaction);
    impl_into_rustweaved_response!(GetSubnetwork);
    impl_into_rustweaved_response!(GetVirtualChainFromBlock);
    impl_into_rustweaved_response!(GetBlocks);
    impl_into_rustweaved_response!(GetBlockCount);
    impl_into_rustweaved_response!(GetBlockDagInfo);
    impl_into_rustweaved_response!(ResolveFinalityConflict);
    impl_into_rustweaved_response!(GetHeaders);
    impl_into_rustweaved_response!(GetUtxosByAddresses);
    impl_into_rustweaved_response!(GetBalanceByAddress);
    impl_into_rustweaved_response!(GetBalancesByAddresses);
    impl_into_rustweaved_response!(GetSinkBlueScore);
    impl_into_rustweaved_response!(Ban);
    impl_into_rustweaved_response!(Unban);
    impl_into_rustweaved_response!(EstimateNetworkHashesPerSecond);
    impl_into_rustweaved_response!(GetMempoolEntriesByAddresses);
    impl_into_rustweaved_response!(GetCoinSupply);
    impl_into_rustweaved_response!(Ping);
    impl_into_rustweaved_response!(GetMetrics);
    impl_into_rustweaved_response!(GetServerInfo);
    impl_into_rustweaved_response!(GetSyncStatus);
    impl_into_rustweaved_response!(GetDaaScoreTimestampEstimate);

    impl_into_kaspad_notify_response!(NotifyBlockAdded);
    impl_into_kaspad_notify_response!(NotifyNewBlockTemplate);
    impl_into_kaspad_notify_response!(NotifyUtxosChanged);
    impl_into_kaspad_notify_response!(NotifyPruningPointUtxoSetOverride);
    impl_into_kaspad_notify_response!(NotifyFinalityConflict);
    impl_into_kaspad_notify_response!(NotifyVirtualDaaScoreChanged);
    impl_into_kaspad_notify_response!(NotifyVirtualChainChanged);
    impl_into_kaspad_notify_response!(NotifySinkBlueScoreChanged);

    impl_into_kaspad_notify_response!(NotifyUtxosChanged, StopNotifyingUtxosChanged);
    impl_into_kaspad_notify_response!(NotifyPruningPointUtxoSetOverride, StopNotifyingPruningPointUtxoSetOverride);

    macro_rules! impl_into_rustweaved_response {
        ($name:tt) => {
            paste::paste! {
                impl_into_rustweaved_response_ex!(kaspa_rpc_core::[<$name Response>],[<$name ResponseMessage>],[<$name Response>]);
            }
        };
        ($core_name:tt, $protowire_name:tt) => {
            paste::paste! {
                impl_into_rustweaved_response_base!(kaspa_rpc_core::[<$core_name Response>],[<$protowire_name ResponseMessage>],[<$protowire_name Response>]);
            }
        };
    }
    use impl_into_rustweaved_response;

    macro_rules! impl_into_rustweaved_response_base {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<RpcResult<$core_struct>> for $protowire_struct {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    item.as_ref().map_err(|x| (*x).clone()).into()
                }
            }

            impl From<RpcError> for $protowire_struct {
                fn from(item: RpcError) -> Self {
                    let x: RpcResult<&$core_struct> = Err(item);
                    x.into()
                }
            }

            impl From<$protowire_struct> for rustweaved_response::Payload {
                fn from(item: $protowire_struct) -> Self {
                    rustweaved_response::Payload::$variant(item)
                }
            }

            impl From<$protowire_struct> for RustweavedResponse {
                fn from(item: $protowire_struct) -> Self {
                    Self { id: 0, payload: Some(rustweaved_response::Payload::$variant(item)) }
                }
            }
        };
    }
    use impl_into_rustweaved_response_base;

    macro_rules! impl_into_rustweaved_response_ex {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<RpcResult<&$core_struct>> for rustweaved_response::Payload {
                fn from(item: RpcResult<&$core_struct>) -> Self {
                    rustweaved_response::Payload::$variant(item.into())
                }
            }

            impl From<RpcResult<&$core_struct>> for RustweavedResponse {
                fn from(item: RpcResult<&$core_struct>) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl From<RpcResult<$core_struct>> for rustweaved_response::Payload {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    rustweaved_response::Payload::$variant(item.into())
                }
            }

            impl From<RpcResult<$core_struct>> for RustweavedResponse {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl_into_rustweaved_response_base!($core_struct, $protowire_struct, $variant);

            // ----------------------------------------------------------------------------
            // protowire to rpc_core
            // ----------------------------------------------------------------------------

            impl TryFrom<&rustweaved_response::Payload> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &rustweaved_response::Payload) -> RpcResult<Self> {
                    if let rustweaved_response::Payload::$variant(response) = item {
                        response.try_into()
                    } else {
                        Err(RpcError::MissingRpcFieldError("Payload".to_string(), stringify!($variant).to_string()))
                    }
                }
            }

            impl TryFrom<&RustweavedResponse> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &RustweavedResponse) -> RpcResult<Self> {
                    item.payload
                        .as_ref()
                        .ok_or(RpcError::MissingRpcFieldError("RustweaveResponse".to_string(), "Payload".to_string()))?
                        .try_into()
                }
            }
        };
    }
    use impl_into_rustweaved_response_ex;

    macro_rules! impl_into_kaspad_notify_response {
        ($name:tt) => {
            impl_into_rustweaved_response!($name);

            paste::paste! {
                impl_into_kaspad_notify_response_ex!(kaspa_rpc_core::[<$name Response>],[<$name ResponseMessage>]);
            }
        };
        ($core_name:tt, $protowire_name:tt) => {
            impl_into_rustweaved_response!($core_name, $protowire_name);

            paste::paste! {
                impl_into_kaspad_notify_response_ex!(kaspa_rpc_core::[<$core_name Response>],[<$protowire_name ResponseMessage>]);
            }
        };
    }
    use impl_into_kaspad_notify_response;

    macro_rules! impl_into_kaspad_notify_response_ex {
        ($($core_struct:ident)::+, $protowire_struct:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl<T> From<Result<(), T>> for $protowire_struct
            where
                T: Into<RpcError>,
            {
                fn from(item: Result<(), T>) -> Self {
                    item
                        .map(|_| $($core_struct)::+{})
                        .map_err(|err| err.into()).into()
                }
            }

        };
    }
    use impl_into_kaspad_notify_response_ex;
}
