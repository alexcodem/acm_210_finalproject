mod graph_loader;
mod graph_analysis;

use graph_loader::load_graph;
use graph_analysis::degree_analysis;
use graph_analysis::display_shortest_paths;
use graph_analysis::clustering_coefficient_summary;

fn main() -> Result<(), Box<dyn std::error::Error>> {let file_path = "amazon0302.txt";
    
    match load_graph(file_path) {
        Ok(graph) => {
            println!("Graph loaded successfully!");
            println!("Number of nodes {}", graph.node_count()); //Prints # of nodes in Graph
            println!("Number of edges {}", graph.edge_count()); //Prints # of edges in Graph
            let(avg_in_degree, avg_out_degree) = degree_analysis(&graph);
            println!("Average In-Degree: {:.4}", avg_in_degree);
            println!("Average Out-Degree: {:.4}", avg_out_degree);

            println!("Preview of shortest paths:");
            let start_node = 0;
            display_shortest_paths(&graph, start_node, 15, "shortest_paths.csv")?;

            clustering_coefficient_summary(&graph, "clustering_coefficients.csv")?;
        }

        Err(e) => {
            eprintln!("Error loading graph! {:?}", e);
        }
    }
    Ok(())
}