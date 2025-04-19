// src/spec.rs

/// A single RPC method’s signature:
///  - `name`: the RPC method (e.g. "eth_getBalance")
///  - `params`: an ordered list of parameter names
#[derive(Debug, Clone)]
pub struct MethodSpec {
    pub name: &'static str,
    pub params: &'static [&'static str],
}

/// Hard‑coded registry of the few methods we care about for now.
/// In the future you could deserialize a JSON file or hook into reth’s types.
pub const RPC_SPECS: &[MethodSpec] = &[
    MethodSpec { name: "eth_blockNumber", params: &[] },
    MethodSpec { name: "eth_getBalance", params: &["address", "block"] },
    MethodSpec { name: "eth_sendTransaction", params: &["tx_object"] },
    MethodSpec { name: "eth_call", params: &["call_object", "block"] },
    // … more …
];
