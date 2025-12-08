use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PrpcError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("RPC error: {0}")]
    Rpc(String),
}

#[derive(Serialize)]
struct RpcRequest {
    jsonrpc: String,
    method: String,
    id: u32,
}

#[derive(Deserialize)]
struct RpcResponse<T> {
    jsonrpc: String,
    result: Option<T>,
    error: Option<RpcError>,
    id: u32,
}

#[derive(Deserialize)]
struct RpcError {
    code: i32,
    message: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Pod {
    pub address: String,
    pub last_seen_timestamp: i64,
    pub pubkey: String,
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct PodsResponse {
    pub pods: Vec<Pod>,
    pub total_count: u32,
}

#[derive(Deserialize, Debug)]
pub struct NodeStats {
    pub active_streams: u32,
    pub cpu_percent: f64,
    pub current_index: u32,
    pub file_size: i64,
    pub last_updated: i64,
    pub packets_received: u32,
    pub packets_sent: u32,
    pub ram_total: i64,
    pub ram_used: i64,
    pub total_bytes: i64,
    pub total_pages: u32,
    pub uptime: i64,
}

pub struct PrpcClient {
    http_client: HttpClient,
    base_url: String,
}

impl PrpcClient {
    pub fn new(ip: &str) -> Self {
        Self {
            http_client: HttpClient::builder()
                .timeout(Duration::from_secs(3))
                .build()
                .unwrap(),
            base_url: format!("http://{}:6000/rpc", ip),
        }
    }

    async fn call<T>(&self, method: &str) -> Result<T, PrpcError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            id: 1,
        };

        let response: RpcResponse<T> = self
            .http_client
            .post(&self.base_url)
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        if let Some(error) = response.error {
            return Err(PrpcError::Rpc(error.message));
        }

        response.result.ok_or_else(|| PrpcError::Rpc("No result in response".to_string()))
    }

    pub async fn get_pods(&self) -> Result<PodsResponse, PrpcError> {
        self.call("get-pods").await
    }

    pub async fn get_stats(&self) -> Result<NodeStats, PrpcError> {
        self.call("get-stats").await
    }
}