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

//Displays shortest paths in our main and writes the entirety to a file
pub fn display_shortest_paths(graph: &DiGraph<u32, ()>, start: u32, max_display: usize, output_file: &str) -> Result<(), std::io::Error> {
    let shortest_paths = compute_shortest_paths_bfs(graph, start);

    //Display a sample of the results
    println!("{:<10}) | {:<10}", "Node", "Cost");
    println!("---------------------");
    for (node, cost) in shortest_paths.iter().take(max_display) {
        println!("{:<10} | {:<10}", node, cost);
    }

    //Write full results to a file
    let file = File::create(output_file)?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "Node,Cost")?;
    for (node, cost) in shortest_paths {
        writeln!(writer, "{},{}", node, cost)?;
    }

    println!("Full shortest paths written to {}", output_file);
    Ok(())
}

//Computers clustering coefficients for a node
pub fn clustering_coefficient(graph: &DiGraph<u32, ()>, node: u32) -> f64 {
    let node_index = graph.node_indices().find(|&i| graph[i] == node).expect("Node not found in Graph!");

    let neighbors: Vec<_> = graph.neighbors_undirected(node_index).collect();
    let neighbor_count = neighbors.len();

    if neighbor_count < 2 {
        return 0.0;
    }

    let mut connected_neighbors = 0;
    for i in 0..neighbor_count {
        for j in (i+1)..neighbor_count {
            if graph.find_edge_undirected(neighbors[i], neighbors[j]).is_some() {
                connected_neighbors += 1;
            }
        }
    }

    let total_possible_connections = neighbors_count * (neighbor_count - 1) / 2;
    connected as f64 / total_possible_connections as f64
}
