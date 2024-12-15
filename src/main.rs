mod graph_loader;
mod graph_analysis;

use graph_loader::load_graph;
use graph_analysis::degree_analysis;

fn main() -> Result<(), Box<dyn std::error::Error>> {let file_path = "amazon0302.txt";
    
    match load_graph(file_path) {
        Ok(graph) => {
            println!("Graph loaded successfully!");
            println!("Number of nodes {}", graph.node_count()); //Prints # of nodes in Graph
            println!("Number of edges {}", graph.edge_count()); //Prints # of edges in Graph

        }

        Err(e) => {
            eprintln!("Error loading graph! {:?}", e);
        }
    }
    Ok(())
}