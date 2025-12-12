use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;
use tokio::sync::mpsc;

#[derive(Error, Debug)]
pub enum PrpcError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("RPC error: {0}")]
    Rpc(String),
    #[error("Node not found on any seed")]
    NodeNotFound,
    #[error("Operation timed out")]
    Timeout,
}

// ... (rest of the structs are the same)

#[derive(Serialize)]
struct RpcRequest {
    jsonrpc: String,
    method: String,
    id: u32,
}

#[derive(Deserialize)]
struct RpcResponse<T> {
    #[allow(dead_code)]
    jsonrpc: String,
    result: Option<T>,
    error: Option<RpcError>,
    #[allow(dead_code)]
    id: u32,
}

#[derive(Deserialize)]
struct RpcError {
    #[allow(dead_code)]
    code: i32,
    message: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Pod {
    pub address: Option<String>,
    pub is_public: Option<bool>,
    pub last_seen_timestamp: i64,
    pub pubkey: Option<String>,
    pub rpc_port: Option<u32>,
    pub storage_committed: Option<i64>,
    pub storage_usage_percent: Option<f64>,
    pub storage_used: Option<i64>,
    pub uptime: Option<i64>,
    pub version: Option<String>,
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

pub const DEFAULT_SEED_IPS: &[&str] = &[
    "173.212.220.65",
    "161.97.97.41",
    "192.190.136.36",
    "192.190.136.38",
    "207.244.255.1",
    "192.190.136.28",
    "192.190.136.29",
    "173.212.203.145",
];

#[derive(Default)]
pub struct FindPNodeOptions {
    pub add_seeds: Option<Vec<String>>,
    pub replace_seeds: Option<Vec<String>>,
    pub timeout_seconds: Option<u64>,
}

impl PrpcClient {
    pub fn new(ip: &str, timeout_seconds: Option<u64>) -> Self {
        let timeout = timeout_seconds.unwrap_or(8);
        Self {
            http_client: HttpClient::builder()
                .timeout(Duration::from_secs(timeout))
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

    pub async fn get_pods_with_stats(&self) -> Result<PodsResponse, PrpcError> {
        self.call("get-pods-with-stats").await
    }

    pub async fn get_stats(&self) -> Result<NodeStats, PrpcError> {
        self.call("get-stats").await
    }
}

pub async fn find_pnode(node_id: &str, options: Option<FindPNodeOptions>) -> Result<Pod, PrpcError> {
    let opts = options.unwrap_or_default();
    let timeout = opts.timeout_seconds.unwrap_or(10);

    let seeds: Vec<String> = if let Some(replace) = opts.replace_seeds {
        replace
    } else {
        let mut default_seeds: Vec<String> = DEFAULT_SEED_IPS.iter().map(|s| s.to_string()).collect();
        if let Some(add) = opts.add_seeds {
            default_seeds.extend(add);
        }
        default_seeds
    };

    let (tx, mut rx) = mpsc::channel::<Pod>(1);

    for seed_ip in seeds {
        let tx = tx.clone();
        let node_id = node_id.to_string();
        tokio::spawn(async move {
            let client = PrpcClient::new(&seed_ip, Some(timeout));
            if let Ok(pods_resp) = client.get_pods().await {
                if let Some(found_pod) = pods_resp.pods.into_iter().find(|p| p.pubkey.as_deref() == Some(&node_id)) {
                    let _ = tx.send(found_pod).await;
                }
            }
        });
    }

    match tokio::time::timeout(Duration::from_secs(timeout), rx.recv()).await {
        Ok(Some(pod)) => Ok(pod),
        _ => Err(PrpcError::NodeNotFound),
    }
}
