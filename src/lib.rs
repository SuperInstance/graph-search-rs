//! # graph-search-rs
//!
//! Graph algorithms implemented in pure Rust with no external dependencies.
//!
//! Provides BFS, DFS, Dijkstra, Bellman-Ford, A*, topological sort,
//! and Tarjan's strongly connected components.

/// A weighted edge in a graph.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edge {
    /// Target node index.
    pub to: usize,
    /// Edge weight (can be negative for Bellman-Ford).
    pub weight: f64,
}

impl Edge {
    /// Create a new edge.
    pub fn new(to: usize, weight: f64) -> Self {
        Self { to, weight }
    }
}

/// Graph represented as adjacency list.
#[derive(Clone, Debug)]
pub struct Graph {
    /// Number of nodes in the graph.
    pub node_count: usize,
    /// Adjacency list: `adj[u]` contains edges from node `u`.
    pub adj: Vec<Vec<Edge>>,
}

impl Graph {
    /// Create a new graph with `n` nodes and no edges.
    pub fn new(n: usize) -> Self {
        Self {
            node_count: n,
            adj: vec![vec![]; n],
        }
    }

    /// Add a directed edge from `u` to `v` with given `weight`.
    pub fn add_directed_edge(&mut self, u: usize, v: usize, weight: f64) {
        assert!(u < self.node_count && v < self.node_count);
        self.adj[u].push(Edge::new(v, weight));
    }

    /// Add an undirected edge between `u` and `v` with given `weight`.
    pub fn add_undirected_edge(&mut self, u: usize, v: usize, weight: f64) {
        self.add_directed_edge(u, v, weight);
        self.add_directed_edge(v, u, weight);
    }
}

/// Breadth-first search returning visitation order from `start`.
///
/// Returns a vector of node indices in BFS order.
pub fn bfs(graph: &Graph, start: usize) -> Vec<usize> {
    let mut visited = vec![false; graph.node_count];
    let mut order = Vec::new();
    let mut queue = std::collections::VecDeque::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        order.push(node);
        for edge in &graph.adj[node] {
            if !visited[edge.to] {
                visited[edge.to] = true;
                queue.push_back(edge.to);
            }
        }
    }
    order
}

/// BFS that computes shortest hop distances from `start`.
///
/// Returns a vector where `dist[v]` is the minimum number of edges from `start`
/// to `v`, or `None` if unreachable.
pub fn bfs_distances(graph: &Graph, start: usize) -> Vec<Option<usize>> {
    let mut dist = vec![None; graph.node_count];
    dist[start] = Some(0);
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        let d = dist[node].unwrap();
        for edge in &graph.adj[node] {
            if dist[edge.to].is_none() {
                dist[edge.to] = Some(d + 1);
                queue.push_back(edge.to);
            }
        }
    }
    dist
}

/// Depth-first search returning visitation order from `start`.
///
/// Returns nodes in DFS preorder.
pub fn dfs(graph: &Graph, start: usize) -> Vec<usize> {
    let mut visited = vec![false; graph.node_count];
    let mut order = Vec::new();
    dfs_recursive(graph, start, &mut visited, &mut order);
    order
}

fn dfs_recursive(graph: &Graph, node: usize, visited: &mut [bool], order: &mut Vec<usize>) {
    visited[node] = true;
    order.push(node);
    for edge in &graph.adj[node] {
        if !visited[edge.to] {
            dfs_recursive(graph, edge.to, visited, order);
        }
    }
}

/// Dijkstra's algorithm for single-source shortest paths.
///
/// Returns a vector of distances from `source`. `None` means unreachable.
/// All edge weights must be non-negative.
pub fn dijkstra(graph: &Graph, source: usize) -> Vec<Option<f64>> {
    let mut dist: Vec<Option<f64>> = vec![None; graph.node_count];
    dist[source] = Some(0.0);

    // Simple priority queue via sorted vec (adequate for correctness)
    let mut pq: Vec<(f64, usize)> = vec![(0.0, source)];

    while let Some(idx) = {
        // Find min
        let mut min_idx = None;
        let mut min_val = f64::INFINITY;
        for (i, &(d, _)) in pq.iter().enumerate() {
            if d < min_val {
                min_val = d;
                min_idx = Some(i);
            }
        }
        min_idx
    } {
        let (d, u) = pq.swap_remove(idx);
        if let Some(cur_d) = dist[u] {
            if d > cur_d {
                continue;
            }
        }
        for edge in &graph.adj[u] {
            let new_dist = d + edge.weight;
            let should_update = match dist[edge.to] {
                None => true,
                Some(old) => new_dist < old,
            };
            if should_update {
                dist[edge.to] = Some(new_dist);
                pq.push((new_dist, edge.to));
            }
        }
    }
    dist
}

/// Dijkstra's algorithm that also returns predecessor information.
///
/// Returns `(distances, predecessors)` where `predecessors[v]` is the node
/// before `v` on the shortest path from `source`, or `None` if `v` is the source or unreachable.
pub fn dijkstra_with_path(graph: &Graph, source: usize) -> (Vec<Option<f64>>, Vec<Option<usize>>) {
    let mut dist: Vec<Option<f64>> = vec![None; graph.node_count];
    let mut pred: Vec<Option<usize>> = vec![None; graph.node_count];
    dist[source] = Some(0.0);

    let mut visited = vec![false; graph.node_count];

    for _ in 0..graph.node_count {
        // Find unvisited node with min distance
        let mut u = None;
        let mut min_d = f64::INFINITY;
        for v in 0..graph.node_count {
            if !visited[v] {
                if let Some(d) = dist[v] {
                    if d < min_d {
                        min_d = d;
                        u = Some(v);
                    }
                }
            }
        }
        let u = match u {
            Some(u) => u,
            None => break,
        };
        visited[u] = true;

        for edge in &graph.adj[u] {
            if visited[edge.to] {
                continue;
            }
            let new_dist = min_d + edge.weight;
            let should_update = match dist[edge.to] {
                None => true,
                Some(old) => new_dist < old,
            };
            if should_update {
                dist[edge.to] = Some(new_dist);
                pred[edge.to] = Some(u);
            }
        }
    }
    (dist, pred)
}

/// Bellman-Ford algorithm for single-source shortest paths.
///
/// Handles negative edge weights. Returns `None` if a negative cycle is reachable from `source`.
/// Otherwise returns distance vector.
pub fn bellman_ford(graph: &Graph, source: usize) -> Option<Vec<Option<f64>>> {
    let n = graph.node_count;
    let mut dist: Vec<Option<f64>> = vec![None; n];
    dist[source] = Some(0.0);

    // Relax all edges n-1 times
    for _ in 0..n.saturating_sub(1) {
        let mut changed = false;
        for u in 0..n {
            if let Some(du) = dist[u] {
                for edge in &graph.adj[u] {
                    let new_dist = du + edge.weight;
                    let should_update = match dist[edge.to] {
                        None => true,
                        Some(old) => new_dist < old,
                    };
                    if should_update {
                        dist[edge.to] = Some(new_dist);
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    // Check for negative cycles
    for u in 0..n {
        if let Some(du) = dist[u] {
            for edge in &graph.adj[u] {
                if let Some(old) = dist[edge.to] {
                    if du + edge.weight < old {
                        return None; // Negative cycle detected
                    }
                }
            }
        }
    }

    Some(dist)
}

/// A* search algorithm.
///
/// `heuristic(v)` should return an admissible estimate of the distance from `v` to the goal.
/// Returns the shortest distance from `start` to `goal`, or `None` if no path exists.
pub fn astar<F>(graph: &Graph, start: usize, goal: usize, heuristic: F) -> Option<f64>
where
    F: Fn(usize) -> f64,
{
    let mut dist: Vec<f64> = vec![f64::INFINITY; graph.node_count];
    let mut closed = vec![false; graph.node_count];
    dist[start] = 0.0;

    for _ in 0..graph.node_count {
        // Find open node with minimum f = dist + heuristic
        let mut u = None;
        let mut min_f = f64::INFINITY;
        for v in 0..graph.node_count {
            if !closed[v] && dist[v] + heuristic(v) < min_f {
                min_f = dist[v] + heuristic(v);
                u = Some(v);
            }
        }
        let u = match u {
            Some(u) => u,
            None => break,
        };
        if u == goal {
            if dist[u].is_finite() {
                return Some(dist[u]);
            }
            return None;
        }
        closed[u] = true;

        for edge in &graph.adj[u] {
            if closed[edge.to] {
                continue;
            }
            let new_dist = dist[u] + edge.weight;
            if new_dist < dist[edge.to] {
                dist[edge.to] = new_dist;
            }
        }
    }

    if dist[goal].is_finite() {
        Some(dist[goal])
    } else {
        None
    }
}

/// Topological sort using Kahn's algorithm.
///
/// Returns `None` if the graph contains a cycle.
/// Otherwise returns nodes in topological order.
pub fn topological_sort(graph: &Graph) -> Option<Vec<usize>> {
    let n = graph.node_count;
    let mut in_degree = vec![0usize; n];
    for u in 0..n {
        for edge in &graph.adj[u] {
            in_degree[edge.to] += 1;
        }
    }

    let mut queue: std::collections::VecDeque<usize> = std::collections::VecDeque::new();
    for (v, &deg) in in_degree.iter().enumerate() {
        if deg == 0 {
            queue.push_back(v);
        }
    }

    let mut order = Vec::new();
    while let Some(u) = queue.pop_front() {
        order.push(u);
        for edge in &graph.adj[u] {
            in_degree[edge.to] -= 1;
            if in_degree[edge.to] == 0 {
                queue.push_back(edge.to);
            }
        }
    }

    if order.len() == n {
        Some(order)
    } else {
        None
    }
}

/// Tarjan's algorithm for strongly connected components.
///
/// Returns a vector of SCCs, each being a vector of node indices.
/// SCCs are returned in reverse topological order.
pub fn tarjan_scc(graph: &Graph) -> Vec<Vec<usize>> {
    let n = graph.node_count;
    let mut index_counter = 0usize;
    let mut indices = vec![None::<usize>; n];
    let mut lowlinks = vec![0usize; n];
    let mut on_stack = vec![false; n];
    let mut stack = Vec::new();
    let mut sccs = Vec::new();

    for v in 0..n {
        if indices[v].is_none() {
            tarjan_strongconnect(
                v,
                &mut index_counter,
                &mut indices,
                &mut lowlinks,
                &mut on_stack,
                &mut stack,
                graph,
                &mut sccs,
            );
        }
    }
    sccs
}

#[allow(clippy::too_many_arguments)]
fn tarjan_strongconnect(
    v: usize,
    index_counter: &mut usize,
    indices: &mut Vec<Option<usize>>,
    lowlinks: &mut Vec<usize>,
    on_stack: &mut Vec<bool>,
    stack: &mut Vec<usize>,
    graph: &Graph,
    sccs: &mut Vec<Vec<usize>>,
) {
    indices[v] = Some(*index_counter);
    lowlinks[v] = *index_counter;
    *index_counter += 1;
    stack.push(v);
    on_stack[v] = true;

    for edge in &graph.adj[v] {
        let w = edge.to;
        if indices[w].is_none() {
            tarjan_strongconnect(w, index_counter, indices, lowlinks, on_stack, stack, graph, sccs);
            lowlinks[v] = lowlinks[v].min(lowlinks[w]);
        } else if on_stack[w] {
            lowlinks[v] = lowlinks[v].min(indices[w].unwrap());
        }
    }

    if lowlinks[v] == indices[v].unwrap() {
        let mut scc = Vec::new();
        loop {
            let w = stack.pop().unwrap();
            on_stack[w] = false;
            scc.push(w);
            if w == v {
                break;
            }
        }
        sccs.push(scc);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_linear_graph(n: usize) -> Graph {
        let mut g = Graph::new(n);
        for i in 0..n.saturating_sub(1) {
            g.add_directed_edge(i, i + 1, 1.0);
        }
        g
    }

    // === BFS Tests ===

    #[test]
    fn test_bfs_linear() {
        let g = make_linear_graph(5);
        let order = bfs(&g, 0);
        assert_eq!(order, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_bfs_single() {
        let g = Graph::new(1);
        assert_eq!(bfs(&g, 0), vec![0]);
    }

    #[test]
    fn test_bfs_disconnected() {
        let mut g = Graph::new(4);
        g.add_directed_edge(0, 1, 1.0);
        g.add_directed_edge(2, 3, 1.0);
        let order = bfs(&g, 0);
        assert_eq!(order, vec![0, 1]); // 2, 3 unreachable
    }

    #[test]
    fn test_bfs_distances() {
        let mut g = Graph::new(4);
        g.add_undirected_edge(0, 1, 1.0);
        g.add_undirected_edge(0, 2, 1.0);
        g.add_undirected_edge(1, 3, 1.0);
        let dist = bfs_distances(&g, 0);
        assert_eq!(dist[0], Some(0));
        assert_eq!(dist[1], Some(1));
        assert_eq!(dist[2], Some(1));
        assert_eq!(dist[3], Some(2));
    }

    #[test]
    fn test_bfs_cycle() {
        let mut g = Graph::new(3);
        g.add_directed_edge(0, 1, 1.0);
        g.add_directed_edge(1, 2, 1.0);
        g.add_directed_edge(2, 0, 1.0);
        let order = bfs(&g, 0);
        assert_eq!(order.len(), 3);
        assert_eq!(order[0], 0);
    }

    // === DFS Tests ===

    #[test]
    fn test_dfs_linear() {
        let g = make_linear_graph(5);
        let order = dfs(&g, 0);
        assert_eq!(order, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_dfs_single() {
        let g = Graph::new(1);
        assert_eq!(dfs(&g, 0), vec![0]);
    }

    #[test]
    fn test_dfs_tree() {
        let mut g = Graph::new(4);
        g.add_directed_edge(0, 1, 1.0);
        g.add_directed_edge(0, 2, 1.0);
        g.add_directed_edge(1, 3, 1.0);
        let order = dfs(&g, 0);
        assert_eq!(order[0], 0);
        // Should visit deep into one branch before the other
        assert_eq!(order.len(), 4);
    }

    #[test]
    fn test_dfs_disconnected() {
        let mut g = Graph::new(4);
        g.add_directed_edge(0, 1, 1.0);
        g.add_directed_edge(2, 3, 1.0);
        let order = dfs(&g, 0);
        assert_eq!(order, vec![0, 1]);
    }

    // === Dijkstra Tests ===

    #[test]
    fn test_dijkstra_simple() {
        let mut g = Graph::new(4);
        g.add_directed_edge(0, 1, 1.0);
        g.add_directed_edge(0, 2, 4.0);
        g.add_directed_edge(1, 2, 2.0);
        g.add_directed_edge(2, 3, 1.0);
        let dist = dijkstra(&g, 0);
        assert_eq!(dist[0], Some(0.0));
        assert_eq!(dist[1], Some(1.0));
        assert_eq!(dist[2], Some(3.0)); // 0->1->2
        assert_eq!(dist[3], Some(4.0));
    }

    #[test]
    fn test_dijkstra_single() {
        let g = Graph::new(1);
        let dist = dijkstra(&g, 0);
        assert_eq!(dist[0], Some(0.0));
    }

    #[test]
    fn test_dijkstra_unreachable() {
        let mut g = Graph::new(3);
        g.add_directed_edge(0, 1, 1.0);
        let dist = dijkstra(&g, 0);
        assert_eq!(dist[0], Some(0.0));
        assert_eq!(dist[1], Some(1.0));
        assert_eq!(dist[2], None);
    }

    #[test]
    fn test_dijkstra_with_path() {
        let mut g = Graph::new(3);
        g.add_directed_edge(0, 1, 1.0);
        g.add_directed_edge(1, 2, 2.0);
        let (dist, pred) = dijkstra_with_path(&g, 0);
        assert_eq!(dist[2], Some(3.0));
        assert_eq!(pred[2], Some(1));
        assert_eq!(pred[1], Some(0));
        assert_eq!(pred[0], None);
    }

    // === Bellman-Ford Tests ===

    #[test]
    fn test_bellman_ford_simple() {
        let mut g = Graph::new(4);
        g.add_directed_edge(0, 1, 1.0);
        g.add_directed_edge(0, 2, 4.0);
        g.add_directed_edge(1, 2, 2.0);
        g.add_directed_edge(2, 3, 1.0);
        let dist = bellman_ford(&g, 0).unwrap();
        assert_eq!(dist[0], Some(0.0));
        assert_eq!(dist[1], Some(1.0));
        assert_eq!(dist[2], Some(3.0));
        assert_eq!(dist[3], Some(4.0));
    }

    #[test]
    fn test_bellman_ford_negative_edges() {
        let mut g = Graph::new(3);
        g.add_directed_edge(0, 1, 1.0);
        g.add_directed_edge(1, 2, -2.0);
        let dist = bellman_ford(&g, 0).unwrap();
        assert_eq!(dist[2], Some(-1.0));
    }

    #[test]
    fn test_bellman_ford_negative_cycle() {
        let mut g = Graph::new(3);
        g.add_directed_edge(0, 1, 1.0);
        g.add_directed_edge(1, 2, -3.0);
        g.add_directed_edge(2, 1, 1.0);
        assert!(bellman_ford(&g, 0).is_none());
    }

    #[test]
    fn test_bellman_ford_unreachable() {
        let g = Graph::new(3);
        let dist = bellman_ford(&g, 0).unwrap();
        assert_eq!(dist[0], Some(0.0));
        assert_eq!(dist[1], None);
    }

    // === A* Tests ===

    #[test]
    fn test_astar_simple() {
        let mut g = Graph::new(4);
        g.add_directed_edge(0, 1, 1.0);
        g.add_directed_edge(0, 2, 4.0);
        g.add_directed_edge(1, 2, 2.0);
        g.add_directed_edge(2, 3, 1.0);
        let dist = astar(&g, 0, 3, |_| 0.0).unwrap(); // Dijkstra-like
        assert!((dist - 4.0).abs() < 1e-9);
    }

    #[test]
    fn test_astar_grid() {
        // 3x3 grid, find path from (0,0) to (2,2)
        let mut g = Graph::new(9);
        for r in 0..3 {
            for c in 0..3 {
                let u = r * 3 + c;
                if c < 2 {
                    g.add_undirected_edge(u, u + 1, 1.0);
                }
                if r < 2 {
                    g.add_undirected_edge(u, u + 3, 1.0);
                }
            }
        }
        let h = |v: usize| {
            let r = v / 3;
            let c = v % 3;
            ((2 - r) as f64 + (2 - c) as f64) as f64
        };
        let dist = astar(&g, 0, 8, h).unwrap();
        assert!((dist - 4.0).abs() < 1e-9);
    }

    #[test]
    fn test_astar_unreachable() {
        let mut g = Graph::new(3);
        g.add_directed_edge(0, 1, 1.0);
        assert_eq!(astar(&g, 0, 2, |_| 0.0), None);
    }

    #[test]
    fn test_astar_start_is_goal() {
        let g = Graph::new(3);
        assert_eq!(astar(&g, 1, 1, |_| 0.0), Some(0.0));
    }

    // === Topological Sort Tests ===

    #[test]
    fn test_topo_linear() {
        let g = make_linear_graph(4);
        let order = topological_sort(&g).unwrap();
        assert_eq!(order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_topo_diamond() {
        let mut g = Graph::new(4);
        g.add_directed_edge(0, 1, 1.0);
        g.add_directed_edge(0, 2, 1.0);
        g.add_directed_edge(1, 3, 1.0);
        g.add_directed_edge(2, 3, 1.0);
        let order = topological_sort(&g).unwrap();
        assert_eq!(order[0], 0);
        assert_eq!(order[3], 3);
        // 1 and 2 can be in either order
    }

    #[test]
    fn test_topo_cycle() {
        let mut g = Graph::new(3);
        g.add_directed_edge(0, 1, 1.0);
        g.add_directed_edge(1, 2, 1.0);
        g.add_directed_edge(2, 0, 1.0);
        assert!(topological_sort(&g).is_none());
    }

    #[test]
    fn test_topo_validity() {
        let mut g = Graph::new(6);
        g.add_directed_edge(5, 2, 1.0);
        g.add_directed_edge(5, 0, 1.0);
        g.add_directed_edge(4, 0, 1.0);
        g.add_directed_edge(4, 1, 1.0);
        g.add_directed_edge(2, 3, 1.0);
        g.add_directed_edge(3, 1, 1.0);
        let order = topological_sort(&g).unwrap();
        // Verify: for every edge u->v, u comes before v
        let pos: Vec<usize> = {
            let mut p = vec![0; 6];
            for (i, &node) in order.iter().enumerate() {
                p[node] = i;
            }
            p
        };
        for u in 0..6 {
            for edge in &g.adj[u] {
                assert!(pos[u] < pos[edge.to], "Invalid topological order");
            }
        }
    }

    // === Tarjan SCC Tests ===

    #[test]
    fn test_tarjan_single_node() {
        let g = Graph::new(1);
        let sccs = tarjan_scc(&g);
        assert_eq!(sccs.len(), 1);
        assert_eq!(sccs[0], vec![0]);
    }

    #[test]
    fn test_tarjan_no_edges() {
        let g = Graph::new(3);
        let sccs = tarjan_scc(&g);
        assert_eq!(sccs.len(), 3);
    }

    #[test]
    fn test_tarjan_single_cycle() {
        let mut g = Graph::new(3);
        g.add_directed_edge(0, 1, 1.0);
        g.add_directed_edge(1, 2, 1.0);
        g.add_directed_edge(2, 0, 1.0);
        let sccs = tarjan_scc(&g);
        assert_eq!(sccs.len(), 1);
        assert_eq!(sccs[0].len(), 3);
    }

    #[test]
    fn test_tarjan_two_sccs() {
        let mut g = Graph::new(4);
        // SCC 1: 0 -> 1 -> 0
        g.add_directed_edge(0, 1, 1.0);
        g.add_directed_edge(1, 0, 1.0);
        // SCC 2: 2 -> 3 -> 2
        g.add_directed_edge(2, 3, 1.0);
        g.add_directed_edge(3, 2, 1.0);
        let sccs = tarjan_scc(&g);
        assert_eq!(sccs.len(), 2);
        for scc in &sccs {
            assert_eq!(scc.len(), 2);
        }
    }

    #[test]
    fn test_tarjan_dag() {
        let g = make_linear_graph(4);
        let sccs = tarjan_scc(&g);
        assert_eq!(sccs.len(), 4); // Each node is its own SCC
    }

    #[test]
    fn test_tarjan_mixed() {
        // 0->1->2->1 (SCC: {1,2}), 0->3 (singleton)
        let mut g = Graph::new(4);
        g.add_directed_edge(0, 1, 1.0);
        g.add_directed_edge(1, 2, 1.0);
        g.add_directed_edge(2, 1, 1.0);
        g.add_directed_edge(0, 3, 1.0);
        let sccs = tarjan_scc(&g);
        // Should have 3 SCCs: {0}, {1,2}, {3}
        assert_eq!(sccs.len(), 3);
        let scc_sizes: Vec<usize> = sccs.iter().map(|s| s.len()).collect();
        assert!(scc_sizes.contains(&2));
        assert_eq!(scc_sizes.iter().filter(|&&s| s == 1).count(), 2);
    }
}
