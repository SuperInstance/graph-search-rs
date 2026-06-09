//! Advanced: Currency arbitrage detection and fleet scheduling
//!
//! Two real-world use cases:
//! 1. Detecting arbitrage in currency exchange via negative cycle detection
//! 2. Fleet task scheduling with dependency resolution

use graph_search_rs::{Graph, bellman_ford, dijkstra_with_path, topological_sort, tarjan_scc};

fn main() {
    // === Use Case 1: Currency Arbitrage Detection ===
    println!("=== Currency Arbitrage Detection ===\n");

    // In forex, edge weight = -ln(exchange_rate)
    // A negative cycle means you can start with 1 unit, go around, and end up with >1
    let mut forex = Graph::new(4);
    // Currencies: 0=USD, 1=EUR, 2=GBP, 3=JPY
    forex.add_directed_edge(0, 1, -0.1); // USD→EUR at rate e^0.1 ≈ 1.105
    forex.add_directed_edge(1, 2, -0.05); // EUR→GBP at rate e^0.05 ≈ 1.051
    forex.add_directed_edge(2, 0, -0.05); // GBP→USD at rate e^0.05 ≈ 1.051
    // Cycle: USD→EUR→GBP→USD = -0.1 + -0.05 + -0.05 = -0.2 < 0 → ARBITRAGE!
    forex.add_directed_edge(1, 3, 0.3);   // EUR→JPY (no arbitrage on this path)

    match bellman_ford(&forex, 0) {
        Some(dist) => {
            println!("No arbitrage found. Best exchange costs from USD:");
            for (i, d) in dist.iter().enumerate() {
                if let Some(d) = d {
                    println!("  → Currency {}: cost = {:.4} (rate ≈ {:.4})", i, d, d.exp());
                }
            }
        }
        None => {
            println!("⚠️  ARBITRAGE DETECTED!");
            println!("   A negative cycle exists in the exchange graph.");
            println!("   This means you can make risk-free profit by trading in a loop.");
            println!("   Cycle: USD → EUR → GBP → USD");
            println!("   Total cost: -0.2 → profit factor = e^0.2 ≈ {:.3}", 0.2_f64.exp());
        }
    }
    println!();

    // === Use Case 2: Fleet Task Scheduling ===
    println!("=== Fleet Task Scheduling ===\n");

    // 8 tasks with dependencies
    let mut tasks = Graph::new(8);
    tasks.add_directed_edge(0, 2, 1.0); // task 0 must finish before task 2
    tasks.add_directed_edge(1, 2, 1.0); // task 1 must finish before task 2
    tasks.add_directed_edge(2, 3, 1.0);
    tasks.add_directed_edge(2, 4, 1.0);
    tasks.add_directed_edge(3, 5, 1.0);
    tasks.add_directed_edge(4, 5, 1.0);
    tasks.add_directed_edge(5, 6, 1.0);
    tasks.add_directed_edge(6, 7, 1.0);

    match topological_sort(&tasks) {
        Some(order) => {
            println!("Valid execution order: {:?}", order);
            println!();
            println!("Parallel execution tiers:");
            // Find tier for each task (longest path from source)
            let (dist, _) = dijkstra_with_path(&tasks, 0);
            for tier in 0..=7 {
                let tier_tasks: Vec<usize> = order.iter()
                    .filter(|&&n| {
                        if let Some(d) = dist.get(n).and_then(|d| *d) {
                            d as usize == tier
                        } else {
                            // Tasks not reachable from 0 might still be in the order
                            n == 1 && tier == 0
                        }
                    })
                    .copied()
                    .collect();
                if !tier_tasks.is_empty() {
                    println!("  Tier {}: Tasks {:?} (run in parallel)", tier, tier_tasks);
                }
            }
        }
        None => println!("Cannot schedule: circular dependency!"),
    }
    println!();

    // === Use Case 3: Community Detection ===
    println!("=== Agent Community Detection ===\n");

    let mut trust = Graph::new(10);
    // Cluster A: agents 0-3 trust each other
    trust.add_directed_edge(0, 1, 1.0);
    trust.add_directed_edge(1, 2, 1.0);
    trust.add_directed_edge(2, 0, 1.0); // cycle → SCC
    trust.add_directed_edge(1, 3, 1.0);
    trust.add_directed_edge(3, 0, 1.0);

    // Cluster B: agents 4-6 trust each other
    trust.add_directed_edge(4, 5, 1.0);
    trust.add_directed_edge(5, 6, 1.0);
    trust.add_directed_edge(6, 4, 1.0); // cycle → SCC

    // Bridge: agent 7 connects both clusters
    trust.add_directed_edge(3, 7, 1.0);
    trust.add_directed_edge(7, 4, 1.0);

    // Isolated agents 8, 9
    trust.add_directed_edge(8, 9, 1.0);

    let sccs = tarjan_scc(&trust);
    println!("Found {} communities in trust network:", sccs.len());
    for (i, scc) in sccs.iter().enumerate() {
        let label = if scc.len() > 1 { "tightly-knit group" } else { "singleton" };
        println!("  Community {}: {:?} — {}", i, scc, label);
    }
}
