use crate::pb::rustweaved_message::Payload as RustweavedMessagePayload;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum RustweavedMessagePayloadType {
    Addresses = 0,
    Block,
    Transaction,
    BlockLocator,
    RequestAddresses,
    RequestRelayBlocks,
    RequestTransactions,
    IbdBlock,
    InvRelayBlock,
    InvTransactions,
    Ping,
    Pong,
    Verack,
    Version,
    TransactionNotFound,
    Reject,
    PruningPointUtxoSetChunk,
    RequestIbdBlocks,
    UnexpectedPruningPoint,
    IbdBlockLocator,
    IbdBlockLocatorHighestHash,
    RequestNextPruningPointUtxoSetChunk,
    DonePruningPointUtxoSetChunks,
    IbdBlockLocatorHighestHashNotFound,
    BlockWithTrustedData,
    DoneBlocksWithTrustedData,
    RequestPruningPointAndItsAnticone,
    BlockHeaders,
    RequestNextHeaders,
    DoneHeaders,
    RequestPruningPointUtxoSet,
    RequestHeaders,
    RequestBlockLocator,
    PruningPoints,
    RequestPruningPointProof,
    PruningPointProof,
    Ready,
    BlockWithTrustedDataV4,
    TrustedData,
    RequestIbdChainBlockLocator,
    IbdChainBlockLocator,
    RequestAntipast,
    RequestNextPruningPointAndItsAnticoneBlocks,
}

impl From<&RustweavedMessagePayload> for RustweavedMessagePayloadType {
    fn from(payload: &RustweavedMessagePayload) -> Self {
        match payload {
            RustweavedMessagePayload::Addresses(_) => RustweavedMessagePayloadType::Addresses,
            RustweavedMessagePayload::Block(_) => RustweavedMessagePayloadType::Block,
            RustweavedMessagePayload::Transaction(_) => RustweavedMessagePayloadType::Transaction,
            RustweavedMessagePayload::BlockLocator(_) => RustweavedMessagePayloadType::BlockLocator,
            RustweavedMessagePayload::RequestAddresses(_) => RustweavedMessagePayloadType::RequestAddresses,
            RustweavedMessagePayload::RequestRelayBlocks(_) => RustweavedMessagePayloadType::RequestRelayBlocks,
            RustweavedMessagePayload::RequestTransactions(_) => RustweavedMessagePayloadType::RequestTransactions,
            RustweavedMessagePayload::IbdBlock(_) => RustweavedMessagePayloadType::IbdBlock,
            RustweavedMessagePayload::InvRelayBlock(_) => RustweavedMessagePayloadType::InvRelayBlock,
            RustweavedMessagePayload::InvTransactions(_) => RustweavedMessagePayloadType::InvTransactions,
            RustweavedMessagePayload::Ping(_) => RustweavedMessagePayloadType::Ping,
            RustweavedMessagePayload::Pong(_) => RustweavedMessagePayloadType::Pong,
            RustweavedMessagePayload::Verack(_) => RustweavedMessagePayloadType::Verack,
            RustweavedMessagePayload::Version(_) => RustweavedMessagePayloadType::Version,
            RustweavedMessagePayload::TransactionNotFound(_) => RustweavedMessagePayloadType::TransactionNotFound,
            RustweavedMessagePayload::Reject(_) => RustweavedMessagePayloadType::Reject,
            RustweavedMessagePayload::PruningPointUtxoSetChunk(_) => RustweavedMessagePayloadType::PruningPointUtxoSetChunk,
            RustweavedMessagePayload::RequestIbdBlocks(_) => RustweavedMessagePayloadType::RequestIbdBlocks,
            RustweavedMessagePayload::UnexpectedPruningPoint(_) => RustweavedMessagePayloadType::UnexpectedPruningPoint,
            RustweavedMessagePayload::IbdBlockLocator(_) => RustweavedMessagePayloadType::IbdBlockLocator,
            RustweavedMessagePayload::IbdBlockLocatorHighestHash(_) => RustweavedMessagePayloadType::IbdBlockLocatorHighestHash,
            RustweavedMessagePayload::RequestNextPruningPointUtxoSetChunk(_) => {
                RustweavedMessagePayloadType::RequestNextPruningPointUtxoSetChunk
            }
            RustweavedMessagePayload::DonePruningPointUtxoSetChunks(_) => RustweavedMessagePayloadType::DonePruningPointUtxoSetChunks,
            RustweavedMessagePayload::IbdBlockLocatorHighestHashNotFound(_) => {
                RustweavedMessagePayloadType::IbdBlockLocatorHighestHashNotFound
            }
            RustweavedMessagePayload::BlockWithTrustedData(_) => RustweavedMessagePayloadType::BlockWithTrustedData,
            RustweavedMessagePayload::DoneBlocksWithTrustedData(_) => RustweavedMessagePayloadType::DoneBlocksWithTrustedData,
            RustweavedMessagePayload::RequestPruningPointAndItsAnticone(_) => RustweavedMessagePayloadType::RequestPruningPointAndItsAnticone,
            RustweavedMessagePayload::BlockHeaders(_) => RustweavedMessagePayloadType::BlockHeaders,
            RustweavedMessagePayload::RequestNextHeaders(_) => RustweavedMessagePayloadType::RequestNextHeaders,
            RustweavedMessagePayload::DoneHeaders(_) => RustweavedMessagePayloadType::DoneHeaders,
            RustweavedMessagePayload::RequestPruningPointUtxoSet(_) => RustweavedMessagePayloadType::RequestPruningPointUtxoSet,
            RustweavedMessagePayload::RequestHeaders(_) => RustweavedMessagePayloadType::RequestHeaders,
            RustweavedMessagePayload::RequestBlockLocator(_) => RustweavedMessagePayloadType::RequestBlockLocator,
            RustweavedMessagePayload::PruningPoints(_) => RustweavedMessagePayloadType::PruningPoints,
            RustweavedMessagePayload::RequestPruningPointProof(_) => RustweavedMessagePayloadType::RequestPruningPointProof,
            RustweavedMessagePayload::PruningPointProof(_) => RustweavedMessagePayloadType::PruningPointProof,
            RustweavedMessagePayload::Ready(_) => RustweavedMessagePayloadType::Ready,
            RustweavedMessagePayload::BlockWithTrustedDataV4(_) => RustweavedMessagePayloadType::BlockWithTrustedDataV4,
            RustweavedMessagePayload::TrustedData(_) => RustweavedMessagePayloadType::TrustedData,
            RustweavedMessagePayload::RequestIbdChainBlockLocator(_) => RustweavedMessagePayloadType::RequestIbdChainBlockLocator,
            RustweavedMessagePayload::IbdChainBlockLocator(_) => RustweavedMessagePayloadType::IbdChainBlockLocator,
            RustweavedMessagePayload::RequestAntipast(_) => RustweavedMessagePayloadType::RequestAntipast,
            RustweavedMessagePayload::RequestNextPruningPointAndItsAnticoneBlocks(_) => {
                RustweavedMessagePayloadType::RequestNextPruningPointAndItsAnticoneBlocks
            }
        }
    }
}
