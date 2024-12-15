mod graph_loader;

use graph_loader::load_graph;

fn main() -> Result<(), Box<dyn std::error::Error>> {let file_path = "amazon0302.txt";
    
    match load_graph(file_path) {
        Ok(_graph) => {
            println!("Graph loaded successfully!");
            println!("Number of nodes {}", graph.nodes_count()); //Prints # of nodes in Graph
            println!("Number of edges {}", graph.edges_count()); //Prints # of edges in Graph

        }

        Err(e) => {
            eprintln!("Error loading graph! {:?}", e);
        }
    }
    Ok(())
}
