use petgraph::graph::DiGraph;
use petgraph::visit::Bfs;
use petgraph::Direction;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufWriter};

pub fn degree_analysis(graph: &DiGraph<u32, ()>) -> (f64, f64) {
    let mut in_degrees = vec![];
    let mut out_degrees = vec![];

    for node in graph.node_indices() {
        in_degrees.push(graph.neighbors_directed(node, Direction::Incoming).count());
        out_degrees.push(graph.neighbors_directed(node, Direction::Outgoing).count());
    }

    let avg_in_degree
    let avg_out_degree

    (avg_indegree, avg_out_degree)
}

fn average(){
    
}