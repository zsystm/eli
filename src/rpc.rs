// src/rpc.rs

use serde::{Deserialize, Serialize};
use serde_json::Value;
use reqwest::Client;
use anyhow::Result;

/// Represents a JSON-RPC request payload.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct JsonRpcRequest {
    /// JSON-RPC version, always "2.0".
    pub jsonrpc: String,
    /// The name of the JSON-RPC method to call.
    pub method: String,
    /// Parameters for the method call, represented as a JSON value.
    pub params: Value,
    /// Unique identifier for the request.
    pub id: u64,
}

impl JsonRpcRequest {
    /// Creates a new JSON-RPC request with the given method, parameters, and identifier.
    pub fn new(method: impl Into<String>, params: Value, id: u64) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            method: method.into(),
            params,
            id,
        }
    }
}

/// Represents a JSON-RPC response payload.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JsonRpcResponse {
    /// JSON-RPC version, always "2.0".
    pub jsonrpc: String,
    /// The result of the call, if successful.
    pub result: Option<Value>,
    /// The error object, if the call failed.
    pub error: Option<Value>,
    /// Identifier matching the request.
    pub id: u64,
}

/// Sends a JSON-RPC request to the specified URL and returns the parsed response.
///
/// # Arguments
///
/// * `url` - The HTTP endpoint of the Ethereum node (e.g., "http://localhost:8545").
/// * `req_body` - The JSON-RPC request payload.

pub async fn send_rpc_request(
    url: &str,
    req_body: JsonRpcRequest,
) -> Result<JsonRpcResponse> {
    // Initialize HTTP client
    let client = Client::new();
    
    // Send POST request with JSON body
    let resp = client
        .post(url)
        .json(&req_body)
        .send()
        .await?;

    // Parse response JSON into JsonRpcResponse
    let rpc_res = resp.json::<JsonRpcResponse>().await?;
    Ok(rpc_res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use mockito::{mock, server_url};
    use tokio;

    /// Test that JsonRpcRequest serializes and deserializes correctly.
    #[test]
    fn request_serializes_and_deserializes() {
        let params = json!(["0xABC", "latest"]);
        let request = JsonRpcRequest::new("eth_getBalance", params.clone(), 42);

        // Validate struct fields
        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.method, "eth_getBalance");
        assert_eq!(request.params, params);
        assert_eq!(request.id, 42);

        // Serialize to JSON string and back
        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: JsonRpcRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, request);
    }

    /// Integration test using a mock server to validate HTTP behavior.
    #[tokio::test]
    async fn send_rpc_request_uses_mock_server() {
        // Set up mock endpoint
        let _m = mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{ "jsonrpc": "2.0", "result": "0x1", "id": 1 }"#)
            .create();

        // Build request object
        let req = JsonRpcRequest::new("eth_blockNumber", json!([]), 1);

        // Call send_rpc_request against mock server URL
        let url = &server_url();
        let response = send_rpc_request(url, req.clone()).await.unwrap();

        // Validate response fields
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.unwrap(), json!("0x1"));
        assert_eq!(response.id, 1);
        assert!(response.error.is_none());
    }
}
