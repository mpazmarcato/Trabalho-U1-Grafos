use rand::Rng;
use std::{fs::File, io, io::Write};

// FIXME: ideally the struct field should be private.
#[derive(Debug, Clone)]
pub struct AdjacencyMatrix(pub Vec<Vec<i32>>);

#[derive(Debug)]
pub struct Node {
    value: usize,
    visited: bool,
    ancestor: Option<usize>,
}

impl AdjacencyMatrix {
    pub fn dfs(&self) -> i32 {
        let mut vertices: Vec<Node> = (0..self.0.len())
            .map(|i| Node {
                value: i,
                visited: false,
                ancestor: None,
            })
            .collect();

        let mut stack: Vec<usize> = Vec::new();
        let initial: usize = rand::rng().random_range(0..self.0.len());

        stack.push(initial);
        vertices[initial].visited = true;

        while let Some(row) = stack.last().copied() {
            let unvisited: Option<usize> = self.0[row]
                .iter()
                .enumerate()
                .find(|&(idx, &val)| val == 1 && !vertices[idx].visited)
                .map(|(i, _)| i);

            if let Some(node) = unvisited {
                vertices[node].visited = true;
                vertices[node].ancestor = Some(row);
                stack.push(node);
            } else {
                stack.pop();
            }
        }
        println!("indo escrever");
        println!("{vertices:?}");
        AdjacencyMatrix::write_graph_to_dot(&vertices, String::from("teste.dot"));

        1
    }

    fn write_graph_to_dot(graph: &Vec<Node>, path: String) -> io::Result<()> {
        let mut file: File = File::create(path)?;

        writeln!(file, "digraph G {{")?;
        writeln!(file, "  rankdir=LR;")?;
        writeln!(file, "  node [shape=circle];")?;

        for node in graph {
            writeln!(file, "  {}", node.value)?;
        }

        for node in graph {
            if let Some(ancestor_idx) = node.ancestor {
                writeln!(file, "  {} -> {};", graph[ancestor_idx].value, node.value)?;
            }
        }

        writeln!(file, " }}")?;
        Ok(())
    }
}

// TODO: Implement the Graph trait
// impl Graph for AdjacencyMatrix {}
