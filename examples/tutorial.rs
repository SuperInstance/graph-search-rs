//! Tutorial: Agent network routing with graph-search-rs
//!
//! Demonstrates building a fleet agent network, finding shortest paths,
//! detecting circular dependencies, and identifying agent communities.

use graph_search_rs::{Graph, bfs_distances, dijkstra, dijkstra_with_path, tarjan_scc, topological_sort};

fn main() {
    println!("=== Fleet Agent Network Routing Tutorial ===\n");

    // Step 1: Build the agent network
    println!("Step 1: Build network (8 agents, weighted edges = latency in ms)");
    let mut net = Graph::new(8);
    net.add_undirected_edge(0, 1, 2.0); // gateway ↔ coordinator
    net.add_undirected_edge(0, 2, 5.0); // gateway ↔ sentinel
    net.add_undirected_edge(1, 3, 3.0); // coordinator ↔ builder_a
    net.add_undirected_edge(1, 4, 1.0); // coordinator ↔ builder_b
    net.add_undirected_edge(2, 5, 4.0); // sentinel ↔ observer
    net.add_undirected_edge(3, 6, 6.0); // builder_a ↔ data_store
    net.add_undirected_edge(4, 6, 2.0); // builder_b ↔ data_store
    net.add_undirected_edge(5, 7, 1.0); // observer ↔ logger
    net.add_undirected_edge(6, 7, 3.0); // data_store ↔ logger
    println!("  8 nodes, 9 edges\n");

    // Step 2: Find shortest weighted paths from gateway
    println!("Step 2: Shortest latency from gateway (node 0)");
    let dist = dijkstra(&net, 0);
    for (node, d) in dist.iter().enumerate() {
        match d {
            Some(d) => println!("  → Agent {}: {:.1}ms", node, d),
            None => println!("  → Agent {}: unreachable", node),
        }
    }
    println!();

    // Step 3: Reconstruct actual path
    println!("Step 3: Path from gateway to logger");
    let (dist2, pred) = dijkstra_with_path(&net, 0);
    let target = 7;
    let mut path = vec![target];
    let mut cur = target;
    while let Some(p) = pred[cur] {
        path.push(p);
        cur = p;
    }
    path.reverse();
    println!("  Path: {:?} ({}ms)\n", path, dist2[target].unwrap());

    // Step 4: Hop counts (unweighted)
    println!("Step 4: Hop counts from gateway");
    let hops = bfs_distances(&net, 0);
    for (node, h) in hops.iter().enumerate() {
        match h {
            Some(h) => println!("  Agent {}: {} hops", node, h),
            None => println!("  Agent {}: unreachable", node),
        }
    }
    println!();

    // Step 5: Dependency ordering
    println!("Step 5: Task dependency ordering");
    let mut tasks = Graph::new(6);
    tasks.add_directed_edge(5, 2, 1.0);
    tasks.add_directed_edge(5, 0, 1.0);
    tasks.add_directed_edge(4, 0, 1.0);
    tasks.add_directed_edge(4, 1, 1.0);
    tasks.add_directed_edge(2, 3, 1.0);
    tasks.add_directed_edge(3, 1, 1.0);
    match topological_sort(&tasks) {
        Some(order) => println!("  Execution order: {:?}", order),
        None => println!("  Circular dependency detected!"),
    }
    println!();

    // Step 6: Community detection
    println!("Step 6: Agent communities (SCC)");
    let mut social = Graph::new(6);
    social.add_directed_edge(0, 1, 1.0);
    social.add_directed_edge(1, 0, 1.0); // {0,1} mutually connected
    social.add_directed_edge(2, 3, 1.0);
    social.add_directed_edge(3, 2, 1.0); // {2,3} mutually connected
    social.add_directed_edge(4, 5, 1.0); // {4}→{5}, one-way
    let sccs = tarjan_scc(&social);
    println!("  Found {} communities:", sccs.len());
    for (i, scc) in sccs.iter().enumerate() {
        println!("    Community {}: {:?} (size: {})", i, scc, scc.len());
    }
}
