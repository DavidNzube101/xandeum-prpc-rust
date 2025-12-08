use xandeum_prpc::PrpcClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test with a working seed IP from the curl test
    let client = PrpcClient::new("173.212.220.65");

    println!("Fetching pods from 173.212.220.65...");
    match client.get_pods().await {
        Ok(response) => {
            println!("Total pods: {}", response.total_count);
            println!("First 5 pods:");
            for pod in response.pods.iter().take(5) {
                let address = pod.address.as_deref().unwrap_or("unknown");
                let pubkey = pod.pubkey.as_deref().unwrap_or("unknown");
                let version = pod.version.as_deref().unwrap_or("unknown");
                println!("  - {} ({}) [{}] v{}, last seen: {}", address, pubkey, version, pod.last_seen_timestamp);
            }
        }
        Err(e) => {
            eprintln!("Error fetching pods: {}", e);
        }
    }

    println!("\nFetching stats from 173.212.220.65...");
    match client.get_stats().await {
        Ok(stats) => {
            println!("CPU: {:.2}%, RAM: {}MB used / {}MB total, Uptime: {}s",
                     stats.cpu_percent,
                     stats.ram_used / 1024 / 1024,
                     stats.ram_total / 1024 / 1024,
                     stats.uptime);
        }
        Err(e) => {
            eprintln!("Error fetching stats: {}", e);
        }
    }

    Ok(())
}