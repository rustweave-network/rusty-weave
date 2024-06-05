use crate::result::Result;
use js_sys::BigInt;
use kaspa_consensus_core::network::{NetworkType, NetworkTypeT};
use wasm_bindgen::prelude::*;
use workflow_wasm::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "bigint | number | HexString")]
    #[derive(Clone, Debug)]
    pub type ISompiToRustweave;
}

/// Convert a Rustweave string to Sompi represented by bigint.
/// This function provides correct precision handling and
/// can be used to parse user input.
/// @category Wallet SDK
#[wasm_bindgen(js_name = "kaspaToSompi")]
pub fn kaspa_to_sompi(kaspa: String) -> Option<BigInt> {
    crate::utils::try_kaspa_str_to_sompi(kaspa).ok().flatten().map(Into::into)
}

///
/// Convert Sompi to a string representation of the amount in Rustweave.
///
/// @category Wallet SDK
///
#[wasm_bindgen(js_name = "sompiToRustweaveString")]
pub fn sompi_to_kaspa_string(sompi: ISompiToRustweave) -> Result<String> {
    let sompi = sompi.try_as_u64()?;
    Ok(crate::utils::sompi_to_kaspa_string(sompi))
}

///
/// Format a Sompi amount to a string representation of the amount in Rustweave with a suffix
/// based on the network type (e.g. `RWE` for mainnet, `TRWE` for testnet,
/// `SRWE` for simnet, `DRWE` for devnet).
///
/// @category Wallet SDK
///
#[wasm_bindgen(js_name = "sompiToRustweaveStringWithSuffix")]
pub fn sompi_to_kaspa_string_with_suffix(sompi: ISompiToRustweave, network: &NetworkTypeT) -> Result<String> {
    let sompi = sompi.try_as_u64()?;
    let network_type = NetworkType::try_from(network)?;
    Ok(crate::utils::sompi_to_kaspa_string_with_suffix(sompi, &network_type))
}
