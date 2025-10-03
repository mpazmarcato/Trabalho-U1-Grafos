use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead, BufReader, Write};

use crate::graph::BfsEvent;
use crate::{DfsEvent, Edge, Graph, UndirectedGraph, graph};

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

    fn write_bfs_tree(&self, start: Node, path: String) -> io::Result<()>
    where
        Self: Sized,
    {
        let mut iter = self.bfs(start);
        let mut file: File = File::create(&path)?;

        writeln!(file, "digraph G {{")?;
        writeln!(file, "  node [shape=circle];")?;

        while let Some(events) = iter.next() {
            for event in events {
                match event {
                    BfsEvent::Discover(node, items) => {
                        writeln!(file, " {} ", node)?;
                        for i in items {
                            writeln!(file, " {} -> {} ", node, i)?;
                        }
                    }
                    BfsEvent::CrossEdge(node_1, node_2) => {
                        writeln!(file, " {} -> {} [style=dashed]; ", node_1, node_2)?
                    }
                }
            }
        }

        writeln!(file, "}}")?;
        Ok(())
    }

    // Permitir somente pra grafos direcionados?
    fn write_dfs_tree_with_edges(&self, start: Node, path: String) -> io::Result<()>
    where
        Self: Sized,
        Node: Copy + Eq + Hash + Display + From<usize>,
    {
        let mut iter = self.classify_edges(start);
        let mut file: File = File::create(&path)?;

        writeln!(file, "digraph G {{")?;
        writeln!(file, "  node [shape=circle];")?;

        while let Some(event) = iter.next() {
            match event {
                Edge::Tree(parent, node) => {
                    writeln!(file, " {} ", node)?;
                    writeln!(file, " {} -> {} ", parent, node)?;
                }
                Edge::Back(node, parent) => writeln!(
                    file,
                    " {} -> {} [color=green, style=dashed]; ",
                    node, parent
                )?,
                Edge::ParentBack(_, _) => continue,
                Edge::Foward(parent, node) => {
                    writeln!(file, " {} -> {} [color=pink, style=dashed]; ", parent, node)?
                }
                Edge::Cross(node_1, node_2) => writeln!(
                    file,
                    " {} -> {} [color=purple, style=dashed]; ",
                    node_1, node_2
                )?,
            }
        }

        writeln!(file, " }}")?;

        Ok(())
    }
}

pub trait UndirectedGraphIO<Node: Copy + Eq + Hash + Display + From<usize>>: GraphIO<Node> {
    fn write_undirected_graph(&self, path: String) -> io::Result<()>
    where
        Self: Sized + UndirectedGraph<Node>,
    {
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

    // TODO: return Result, not Self
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

    fn write_dfs_tree(&self, start: Node, path: String) -> io::Result<()>
    where
        Self: Sized + UndirectedGraph<Node>,
        Node: Copy + Eq + Hash + Display + From<usize> + PartialOrd,
    {
        // TODO: adicionar validação pra não executar se o node for out of bounds?

        let mut iter = self.dfs(start);
        let mut file: File = File::create(&path)?;
        let mut visited_edges: HashSet<(Node, Node)> = HashSet::new();

        writeln!(file, "graph G {{")?;
        writeln!(file, "  node [shape=circle];")?;

        while let Some(event) = iter.next() {
            match event {
                DfsEvent::Discover(node, option) => {
                    writeln!(file, " {} ", node)?;
                    if let Some(parent) = option {
                        let edge = if parent < node {
                            (parent, node)
                        } else {
                            (node, parent)
                        };
                        if visited_edges.insert(edge) {
                            writeln!(file, " {} -- {} ", parent, node)?;
                        }
                    }
                }
                DfsEvent::NonTreeEdge(node, parent) => {
                    let edge = if parent < node {
                        (parent, node)
                    } else {
                        (node, parent)
                    };
                    if visited_edges.insert(edge) {
                        writeln!(file, " {} -- {} [style=dashed];", node, parent)?;
                    }
                }
                DfsEvent::Finish(_) => continue,
            }
        }

        writeln!(file, " }}")?;

        Ok(())
    }

    //FIXME: nome provisório só pra diferenciar
    fn undirected_write_bfs_tree(&self, start: Node, path: String) -> io::Result<()>
    where
        Self: Sized + UndirectedGraph<Node>,
    {
        let mut iter = self.bfs(start);
        let mut file: File = File::create(&path)?;
        let mut visited_edges: HashSet<(Node, Node)> = HashSet::new();

        writeln!(file, "graph G {{")?;
        writeln!(file, "  node [shape=circle];")?;

        while let Some(events) = iter.next() {
            for event in events {
                match event {
                    BfsEvent::Discover(node, items) => {
                        writeln!(file, " {} ", node)?;
                        for i in items {
                            if visited_edges.insert((node, i)) {
                                writeln!(file, " {} -- {} ", node, i)?;
                            }
                        }
                    }
                    BfsEvent::CrossEdge(node_1, node_2) => {
                        if !visited_edges.contains(&(node_1, node_2))
                            && !visited_edges.contains(&(node_2, node_1))
                        {
                            writeln!(file, " {} -- {} [style=dashed]; ", node_1, node_2)?;
                            visited_edges.insert((node_1, node_2));
                        }
                    }
                }
            }
        }

        writeln!(file, "}}")?;
        Ok(())
    }
}
