use kaspa_notify::{scope::Scope, subscription::Command};

use crate::protowire::{
    rustweaved_request, rustweaved_response, RustweavedRequest, RustweavedResponse, NotifyBlockAddedRequestMessage,
    NotifyFinalityConflictRequestMessage, NotifyNewBlockTemplateRequestMessage, NotifyPruningPointUtxoSetOverrideRequestMessage,
    NotifySinkBlueScoreChangedRequestMessage, NotifyUtxosChangedRequestMessage, NotifyVirtualChainChangedRequestMessage,
    NotifyVirtualDaaScoreChangedRequestMessage,
};

impl RustweavedRequest {
    pub fn from_notification_type(scope: &Scope, command: Command) -> Self {
        RustweavedRequest { id: 0, payload: Some(rustweaved_request::Payload::from_notification_type(scope, command)) }
    }

    pub fn is_subscription(&self) -> bool {
        self.payload.as_ref().is_some_and(|x| x.is_subscription())
    }
}

impl rustweaved_request::Payload {
    pub fn from_notification_type(scope: &Scope, command: Command) -> Self {
        match scope {
            Scope::BlockAdded(_) => {
                rustweaved_request::Payload::NotifyBlockAddedRequest(NotifyBlockAddedRequestMessage { command: command.into() })
            }
            Scope::NewBlockTemplate(_) => {
                rustweaved_request::Payload::NotifyNewBlockTemplateRequest(NotifyNewBlockTemplateRequestMessage {
                    command: command.into(),
                })
            }

            Scope::VirtualChainChanged(ref scope) => {
                rustweaved_request::Payload::NotifyVirtualChainChangedRequest(NotifyVirtualChainChangedRequestMessage {
                    command: command.into(),
                    include_accepted_transaction_ids: scope.include_accepted_transaction_ids,
                })
            }
            Scope::FinalityConflict(_) => {
                rustweaved_request::Payload::NotifyFinalityConflictRequest(NotifyFinalityConflictRequestMessage {
                    command: command.into(),
                })
            }
            Scope::FinalityConflictResolved(_) => {
                rustweaved_request::Payload::NotifyFinalityConflictRequest(NotifyFinalityConflictRequestMessage {
                    command: command.into(),
                })
            }
            Scope::UtxosChanged(ref scope) => rustweaved_request::Payload::NotifyUtxosChangedRequest(NotifyUtxosChangedRequestMessage {
                addresses: scope.addresses.iter().map(|x| x.into()).collect::<Vec<String>>(),
                command: command.into(),
            }),
            Scope::SinkBlueScoreChanged(_) => {
                rustweaved_request::Payload::NotifySinkBlueScoreChangedRequest(NotifySinkBlueScoreChangedRequestMessage {
                    command: command.into(),
                })
            }
            Scope::VirtualDaaScoreChanged(_) => {
                rustweaved_request::Payload::NotifyVirtualDaaScoreChangedRequest(NotifyVirtualDaaScoreChangedRequestMessage {
                    command: command.into(),
                })
            }
            Scope::PruningPointUtxoSetOverride(_) => {
                rustweaved_request::Payload::NotifyPruningPointUtxoSetOverrideRequest(NotifyPruningPointUtxoSetOverrideRequestMessage {
                    command: command.into(),
                })
            }
        }
    }

    pub fn is_subscription(&self) -> bool {
        use crate::protowire::rustweaved_request::Payload;
        matches!(
            self,
            Payload::NotifyBlockAddedRequest(_)
                | Payload::NotifyVirtualChainChangedRequest(_)
                | Payload::NotifyFinalityConflictRequest(_)
                | Payload::NotifyUtxosChangedRequest(_)
                | Payload::NotifySinkBlueScoreChangedRequest(_)
                | Payload::NotifyVirtualDaaScoreChangedRequest(_)
                | Payload::NotifyPruningPointUtxoSetOverrideRequest(_)
                | Payload::NotifyNewBlockTemplateRequest(_)
                | Payload::StopNotifyingUtxosChangedRequest(_)
                | Payload::StopNotifyingPruningPointUtxoSetOverrideRequest(_)
        )
    }
}

impl RustweavedResponse {
    pub fn is_notification(&self) -> bool {
        match self.payload {
            Some(ref payload) => payload.is_notification(),
            None => false,
        }
    }
}

#[allow(clippy::match_like_matches_macro)]
impl rustweaved_response::Payload {
    pub fn is_notification(&self) -> bool {
        use crate::protowire::rustweaved_response::Payload;
        match self {
            Payload::BlockAddedNotification(_) => true,
            Payload::VirtualChainChangedNotification(_) => true,
            Payload::FinalityConflictNotification(_) => true,
            Payload::FinalityConflictResolvedNotification(_) => true,
            Payload::UtxosChangedNotification(_) => true,
            Payload::SinkBlueScoreChangedNotification(_) => true,
            Payload::VirtualDaaScoreChangedNotification(_) => true,
            Payload::PruningPointUtxoSetOverrideNotification(_) => true,
            Payload::NewBlockTemplateNotification(_) => true,
            _ => false,
        }
    }
}
