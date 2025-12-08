# xandeum-prpc-rust

A Rust client for interacting with Xandeum pNode pRPC APIs.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
xandeum-prpc = { path = "../xandeum-prpc-rust" }  # or from crates.io when published
```

## Usage

```rust
use xandeum_prpc::PrpcClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PrpcClient::new("173.212.220.65");

    // Get list of pods
    let pods = client.get_pods().await?;
    println!("Found {} pods", pods.total_count);

    // Get stats for a node
    let stats = client.get_stats().await?;
    println!("CPU usage: {:.2}%", stats.cpu_percent);

    Ok(())
}
```

## Running the Example

```bash
cargo run --example get_pods
```

## API

- `PrpcClient::new(ip: &str)` - Create client for a pNode IP
- `get_pods() -> Result<PodsResponse>` - Get list of pods in gossip
- `get_stats() -> Result<NodeStats>` - Get statistics for the node