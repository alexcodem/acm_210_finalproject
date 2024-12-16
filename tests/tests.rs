use acm_210_finalproject::graph_loader::{load_graph, MyError};
use acm_210_finalproject::graph_analysis::degree_analysis;
use acm_210_finalproject::graph_analysis::clustering_coefficient;
use petgraph::graph::DiGraph;

#[test]
fn test_load_valid_graph() {
    let dataset = "0\t1\n1\t2\n2\t0";
    let file_path = "test_dataset.txt";
    std::fs::write(file_path, dataset).expect("Failed to write test to dataset!");
    
    let graph = load_graph(file_path).expect("Failed to load graph!");
    assert_eq!(graph.node_count(), 3, "Expected 3 nodes in the graph");
    assert_eq!(graph.edge_count(), 3, "Expected 3 edges in the graph");

    std::fs::remove_file(file_path).expect("Failed to clean up test dataset");
}

#[test]
fn test_load_graph_empty_file() {
    let file_path = "empty_dataset.txt";
    std::fs::write(file_path, "").expect("Failed to write empty test dataset");

    let result = load_graph(file_path);
    assert!(result.is_ok(), "Graph loading failed or empty dataset");
    let graph = result.unwrap();
    assert_eq!(graph.node_count(), 0, "Expctected 0 nodes for an empty dataset!");
    assert_eq!(graph.edge_count(), 0, "Expctected 0 edges for an empty dataset!");
}

#[test]
fn test_load_graph_malformed_file() {
    let malformed_data = "0\t1\ninvalid_line\n2\t3";
    let file_path = "malformed_dataset.txt";
    std::fs::write(file_path, malformed_data).expect("Failed to write malformed dataset");

    let result = load_graph(file_path);
    assert!(
        matches!(result, Err(MyError::FormatError(_))),
        "Expected FormatError for malformed dataset"
    );

    std::fs::remove_file(file_path).expect("Failed to clean up malformed dataset")
}

#[test]
fn test_load_large_graph() {
    let mut dataset = String::new();
    for i in 0..1000 {
        dataset.push_str(&format!("{}\t{}\n", i, i + 1));
    }

    let file_path = "large_datasetset.txt";
    std::fs::write(file_path, dataset).expect("Failed to write large test dataset");

    let graph = load_graph(file_path).expect("Failed to load large graph");
    assert_eq!(graph.node_count(), 1001, "Expected 1001 nodes in the graph");
    assert_eq!(graph.edge_count(), 1000, "Expected 1000 edges in the graph");

    std::fs::remove_file(file_path).expect("Failed to clean up large dataset");
}

#[test]
fn test_degree_analysis_small_graph() {
    let mut graph = DiGraph::<u32, ()>::new();
    let a = graph.add_node(1);
    let b = graph.add_node(2);
    let c = graph.add_node(3);

    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());
    graph.add_edge(c, a, ());

    let mut in_degrees = vec![];
    let mut out_degrees = vec![];

    for node in graph.node_indices() {
        in_degrees.push(graph.neighbors_directed(node, petgraph::Direction::Incoming).count());
        out_degrees.push(graph.neighbors_directed(node, petgraph::Direction::Outgoing).count());
    }
    
    assert_eq!(in_degrees, vec![1, 1, 1], "Each node should have in-degree 1");
    assert_eq!(out_degrees, vec![1, 1, 1], "Each node should have out-degree 1");
}

#[test]
fn test_graph_analysis_degrees() {
    let mut graph = DiGraph::<u32, ()>::new();
    let a = graph.add_node(1);
    let b = graph.add_node(2);
    let c = graph.add_node(2);

    let (avg_in_degree, avg_out_degree) = degree_analysis(&graph);

    assert_eq!(avg_in_degree, 1.0, "Expected average in-degree to be 1.0");
    assert_eq!(avg_out_degree, 1.0, "Expected average out-degree to be 1.0");
}

#[test]
fn test_clustering_coefficient_no_neighbors() {
    let mut graph = DiGraph::<u32, ()>::new();
    let node = graph.add_node(1);

    let coeff = clustering_coefficient(&graph, graph[node]);
    assert_eq!(coeff, 0.0, "Clustering coefficient with no neighbors should be 0.0");
}

#[test]
fn test_clustering_coefficient_complete_neighbors() {
    let mut graph = DiGraph::<u32, ()>::new();
    let a = graph.add_node(1);
    let b = graph.add_node(2);
    let c = graph.add_node(3);

    graph.add_edge(a, c, ());
    graph.add_edge(b, c, ());
    graph.add_edge(c, a, ());

    let coeff = clustering_coefficient(&graph, graph[a]);
    assert_eq!(coeff, 1.0, "Clustering coefficient for fully connected neigbors should be 1.0");
}

#[test]
fn test_clustering_coefficient_partial_neighbors() {
    let mut graph = DiGraph::<u32, ()>::new();
    let a = graph.add_node(1);
    let b = graph.add_node(2);
    let c = graph.add_node(3);

    graph.add_edge(a, b, ());
    graph.add_edge(a, c, ());

    let coeff = clustering_coefficient(&graph, graph[a]);
    assert_eq!(coeff, 0.0, "Clustering coefficient for partially connected neighbors should be 0.0");
}