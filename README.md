# xandeum-prpc-rust

A Rust client for interacting with Xandeum pNode pRPC APIs.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
xandeum-prpc = "0.1.4" 
```

## Usage

### Basic Usage
```rust
use xandeum_prpc::PrpcClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PrpcClient::new("173.212.220.65", None); // Replace with a pNode IP

    // Get pods with detailed statistics
    let pods_with_stats = client.get_pods_with_stats().await?;
    println!("Found {} pods with stats", pods_with_stats.total_count);

    for pod in pods_with_stats.pods {
        println!("  Pubkey: {:?}, Address: {:?}, Uptime: {:?}, Storage Used: {:?} bytes",
            pod.pubkey, pod.address, pod.uptime, pod.storage_used);
    }

    Ok(())
}
```

### Finding a pNode
The library includes a helper function to concurrently search a list of seed nodes to find a specific pNode by its public key.

```rust
use xandeum_prpc::{find_pnode, FindPNodeOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Find a node using the default seed list
    let pod = find_pnode("2asTHq4vVGazKrmEa3YTXKuYiNZBdv1cQoLc1Tr2kvaw", None).await?;
    println!("Found pod: {:?}", pod);

    // Find a node using a custom seed list and timeout
    let options = FindPNodeOptions {
        replace_seeds: Some(vec!["192.190.136.28".to_string()]),
        timeout_seconds: Some(5),
        ..Default::default()
    };
    let pod_on_custom_seed = find_pnode("GCoCP7CLvVivuWUH1sSA9vMi9jjaJcXpMwVozMVA6yBg", Some(options)).await?;
    println!("Found pod on custom seed: {:?}", pod_on_custom_seed);

    Ok(())
}
```

The default seed IPs are:
```
"173.212.220.65", "161.97.97.41", "192.190.136.36", "192.190.136.38", 
"207.244.255.1", "192.190.136.28", "192.190.136.29", "173.212.203.145"
```

## API

-   `PrpcClient::new(ip: &str, timeout_seconds: Option<u64>)` - Create client for a pNode IP. The optional `timeout_seconds` sets the HTTP timeout (default: 8).
-   `find_pnode(node_id: &str, options: Option<FindPNodeOptions>) -> Result<Pod>` - Concurrently searches seed IPs to find a pNode by its public key.
-   `get_pods() -> Result<PodsResponse>` - Get list of pods in gossip. (Note: Use `get_pods_with_stats` for more data).
-   `get_pods_with_stats() -> Result<PodsResponse>` - Get list of pods with detailed statistics.
-   `get_stats() -> Result<NodeStats>` - Get statistics for a single node.
