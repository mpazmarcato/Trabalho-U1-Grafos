use std::fmt::Display;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead, BufReader, Write};

use crate::{Graph, UndirectedGraph};

pub trait GraphIO<Node: Copy + Eq + Hash + Display + From<usize>>: Graph<Node> {
    fn write_graph(&self, path: String) -> io::Result<()> {
        let mut file: File = File::create(&path)?;

        writeln!(file, "digraph G {{")?;
        writeln!(file, "  rankdir=LR;")?;
        writeln!(file, "  node [shape=circle];")?;

        for node in self.nodes() {
            writeln!(file, " {} ", node)?;
            for neighbor in self.neighbors(node) {
                writeln!(file, " {} -> {} ", node, neighbor)?;
            }
        }

        writeln!(file, " }}")?;

        Ok(())
    }

    fn write_undirected_graph(&self, path: String) -> io::Result<()> {
        let mut file: File = File::create(&path)?;

        let mut visited: Vec<Node> = vec![];

        writeln!(file, "graph G {{")?;
        writeln!(file, "  rankdir=LR;")?;
        writeln!(file, "  node [shape=circle];")?;

        for node in self.nodes() {
            writeln!(file, " {} ", node)?;
            for neighbor in self.neighbors(node) {
                if !visited.contains(&neighbor) {
                    writeln!(file, " {} -- {} ", node, neighbor)?;
                }
            }
            visited.push(node);
        }

        writeln!(file, " }}")?;

        Ok(())
    }

    fn from_file(path: String) -> Self
    where
        Self: Sized,
    {
        let mut graph = Self::new_empty();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        for (idx, line) in reader.lines().enumerate() {
            let content = line.unwrap();
            if content.is_empty() {
                continue;
            }

            if idx == 0 {
                let n: usize = content.parse().unwrap();
                for i in 0..n {
                    graph.add_node(Node::from(i));
                }
            } else {
                let values: Vec<usize> = content
                    .split(',')
                    .map(|x| match x.parse::<usize>() {
                        Ok(value) => value - 1,
                        Err(_) => panic!("Graph can't have non-integer nodes!"),
                    })
                    .collect();
                graph.add_edge(Node::from(values[0]), Node::from(values[1]));
            }
        }

        graph
    }

    fn undirected_from_file(path: String) -> Self
    where
        Self: Sized + UndirectedGraph<Node>,
    {
        let mut graph = Self::new_empty();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        for (idx, line) in reader.lines().enumerate() {
            let content = line.unwrap();
            if content.is_empty() {
                continue;
            }

            if idx == 0 {
                let n: usize = content.parse().unwrap();
                for i in 0..n {
                    graph.add_node(Node::from(i));
                }
            } else {
                let values: Vec<usize> = content
                    .split(',')
                    .map(|x| match x.parse::<usize>() {
                        Ok(value) => value - 1,
                        Err(_) => panic!("Graph can't have non-integer nodes!"),
                    })
                    .collect();
                graph.add_undirected_edge(Node::from(values[0]), Node::from(values[1]));
            }
        }

        graph
    }
}
