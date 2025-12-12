use xandeum_prpc::{find_pnode, PrpcClient, FindPNodeOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- Usage 1: Create a client for a specific pNode IP ---
    // The second argument is an optional timeout in seconds.
    // We pass `None` to use the default timeout (8 seconds).
    let client = PrpcClient::new("173.212.220.65", None);

    println!("Fetching stats from a single pNode (173.212.220.65)...");
    match client.get_stats().await {
        Ok(stats) => {
            println!(
                "  - CPU: {:.2}%, RAM: {}MB used / {}MB total, Uptime: {}s",
                stats.cpu_percent,
                stats.ram_used / 1024 / 1024,
                stats.ram_total / 1024 / 1024,
                stats.uptime
            );
        }
        Err(e) => {
            eprintln!("  - Error fetching stats: {}", e);
        }
    }

    // --- Usage 2: Find a specific pNode using the default seed list ---
    println!("\nFinding a specific pNode across all default seed IPs...");
    // Use the `find_pnode` helper to concurrently search all seed nodes.
    // This is much more efficient than creating a client for each seed IP yourself.
    let node_id_to_find = "HjeRsvpPX4CnJAXW3ua2y1qrRA7t9nf8s4dYgJnavQnC";
    match find_pnode(node_id_to_find, None).await {
        Ok(pod) => {
            println!("  - Successfully found pNode {}!", node_id_to_find);
            println!("    - Address: {}", pod.address.unwrap_or_default());
            println!("    - Version: {}", pod.version.unwrap_or_default());
        }
        Err(e) => {
            eprintln!("  - Error finding pNode {}: {}", node_id_to_find, e);
        }
    }

    // --- Usage 3: Find a pNode using a custom (replaced) seed list ---
    println!("\nFinding a pNode using a custom seed list...");
    let options = FindPNodeOptions {
        replace_seeds: Some(vec!["192.190.136.28".to_string()]),
        ..Default::default()
    };
    let node_id_to_find_2 = "GCoCP7CLvVivuWUH1sSA9vMi9jjaJcXpMwVozMVA6yBg";
     match find_pnode(node_id_to_find_2, Some(options)).await {
        Ok(pod) => {
            println!("  - Successfully found pNode {} on custom seed!", node_id_to_find_2);
            println!("    - Address: {}", pod.address.unwrap_or_default());
        }
        Err(e) => {
            eprintln!("  - Error finding pNode {}: {}", node_id_to_find_2, e);
        }
    }

    Ok(())
}
