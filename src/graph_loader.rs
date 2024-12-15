use std::fs::File;
use petgraph::graph::DiGraph;
use std::num::ParseIntError;
use std::io::{self, BufRead, BufReader};
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    IoError(io::Error),
    ParseError(ParseIntError),
    FormatError(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::IoError(err) => write!(f, "I/O Error!: {}", err),
            MyError::ParseError(err) => write!(f, "Parse Error!: {}", err),
            MyError::FormatError(msg) => write!(f, "Format Error!: {},", msg),
        }
    }
}

impl std::error::Error for MyError {}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError::IoError(error)
    }
}

impl From<ParseIntError> for MyError {
    fn from(error: ParseIntError) -> Self {
        MyError::ParseError(error)
    }
}

pub fn load_graph(file_path: &str) -> Result<DiGraph<u32, ()>, MyError> {
    let mut graph = DiGraph::<u32, ()>::new();
    let mut node_map = std::collections::HashMap::new();

    let file = File::open(file_path).map_err(MyError::IoError)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.map_err(MyError::IoError)?;

        if line.starts_with("#") {
            continue;
        }

        let nodes: Vec<&str> = line.split('\t').collect();
        if nodes.len() != 2 {
            return Err(MyError::FormatError(format!("Invalid line format: {}", line)));
        }

        let from_node: u32 = nodes[0].parse().map_err(MyError::ParseError)?;
        let to_node: u32 = nodes[1].parse().map_err(MyError::ParseError)?;
        
        let from_index = *node_map.entry(from_node).or_insert_with( || graph.add_node(from_node));
        let to_index = *node_map.entry(to_node).or_insert_with( || graph.add_node(to_node));
        graph.add_edge(from_index, to_index, ());
    }
    Ok(graph)
}