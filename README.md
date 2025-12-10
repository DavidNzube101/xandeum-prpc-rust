# xandeum-prpc-rust

A Rust client for interacting with Xandeum pNode pRPC APIs.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
xandeum-prpc = "0.1.4" 
```

## Usage

```rust
use xandeum_prpc::PrpcClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PrpcClient::new("173.212.220.65"); // Replace with a pNode IP

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

## API

-   `PrpcClient::new(ip: &str)` - Create client for a pNode IP.
-   `get_pods() -> Result<PodsResponse>` - Get list of pods in gossip. (Note: Use `get_pods_with_stats` for more data).
-   `get_pods_with_stats() -> Result<PodsResponse>` - Get list of pods with detailed statistics *introduced in 0.1.4*  .
-   `get_stats() -> Result<NodeStats>` - Get statistics for a single node.
