use rand::Rng;
use std::{fs::File, io, io::Write};

// FIXME: ideally the struct field should be private.
#[derive(Debug, Clone)]
pub struct AdjacencyMatrix(pub Vec<Vec<i32>>);

#[derive(Debug)]
pub struct Node {
    value: i32,
    visited: bool,
    ancestor: i32,
}

impl AdjacencyMatrix {
    pub fn dfs(graph: Vec<Vec<i32>>) -> i32 {
        let mut vertices: Vec<Node> = Vec::new();
        for i in 0..graph.len() {
            vertices.push(Node {
                value: i as i32,
                visited: false,
                ancestor: -1,
            });
        }

        let mut stack: Vec<usize> = Vec::new();
        let initial: usize = rand::rng().random_range(0..graph.len());
        stack.push(initial);
        vertices[initial].visited = true;

        while let Some(&row) = stack.last() {
            let mut found_unvisited = false;

            for (i, &val) in graph[row].iter().enumerate() {
                if val == 1 {
                    if let Some(vertex) = vertices.iter_mut().find(|v| v.value == i as i32) {
                        if !vertex.visited {
                            vertex.visited = true;
                            vertex.ancestor = row as i32;
                            stack.push(i);
                            found_unvisited = true;
                            break;
                        }
                    }
                }
            }
            if !found_unvisited {
                stack.pop();
            }
        }
        println!("indo escrever");
        println!("{vertices:?}");
        AdjacencyMatrix::write_graph_to_dot(vertices, String::from("teste.dot"));

        1
    }

    fn write_graph_to_dot(graph: Vec<Node>, path: String) -> io::Result<()> {
        let mut file: File = File::create(path)?;

        writeln!(file, "digraph G {{")?;
        writeln!(file, "  rankdir=LR;")?;
        writeln!(file, "  node [shape=circle];")?;

        for node in &graph {
            writeln!(file, "  {}", node.value)?;
        }

        for (i, node) in graph.iter().enumerate() {
            if node.ancestor != -1 && i != (node.ancestor as i32).try_into().unwrap() {
                writeln!(
                    file,
                    "  {} -> {};",
                    graph[node.ancestor as usize].value, node.value
                )?;
            }
        }

        writeln!(file, " }}")?;
        Ok(())
    }
}

// TODO: Implement the Graph trait
// impl Graph for AdjacencyMatrix {}
