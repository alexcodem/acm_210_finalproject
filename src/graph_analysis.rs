use petgraph::graph::DiGraph;
use petgraph::visit::Bfs;
use petgraph::Direction;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufWriter};

//Analyzes the degree distribution of our directed graph
pub fn degree_analysis(graph: &DiGraph<u32, ()>) -> (f64, f64) {
    let mut in_degrees = vec![];
    let mut out_degrees = vec![];

    for node in graph.node_indices() {
        in_degrees.push(graph.neighbors_directed(node, Direction::Incoming).count());
        out_degrees.push(graph.neighbors_directed(node, Direction::Outgoing).count());
    }

    let avg_in_degree = average(&in_degrees);
    let avg_out_degree = average(&out_degrees);

    (avg_in_degree, avg_out_degree)
}

//Helper function to calculate the average of our degrees
fn average(degrees: &[usize]) -> f64 {
    if degrees.is_empty() {
        return 0.0;
    }
    degrees.iter().copied().sum::<usize>() as f64 / degrees.len() as f64
}

pub fn compute_shortest_paths_bfs(graph: &DiGraph<u32, ()>, start: u32) -> HashMap<u32,u32> {
    let start_index = graph.node_indices().find(|&i| graph[i] == start).expect("Start node not found in Graph!");

    let mut bfs = Bfs::new(&graph, start_index);
    let mut distances = HashMap::new();
    distances.insert(start, 0);

    while let Some(node) = bfs.next(&graph) {
        let current_distance = distances[&graph[node]];

        for neighbor in graph.neighbors(node) {
            if !distances.contains_key(&graph[neighbor]) {
                distances.insert(graph[neighbor], current_distance + 1);
            }
        }
    }
    distances
}
