use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};

use crate::graph::BfsEvent;
use crate::{DfsEvent, Edge, Graph, UndirectedGraph};

pub trait GraphIO<Node: Copy + Eq + Hash + Display + From<usize>>: Graph<Node> {
    fn import_from_file(path: String) -> Result<Self, Error>
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
                    .map(|x| {
                        x.trim().parse::<usize>().map(|v| v - 1).map_err(|_| {
                            Error::new(
                                ErrorKind::InvalidData,
                                format!("Invalid data was found during file creation: {} ", x),
                            )
                        })
                    })
                    .collect::<Result<Vec<_>, Error>>()?;
                graph.add_edge(Node::from(values[0]), Node::from(values[1]));
            }
        }

        Ok(graph)
    }

    fn export_to_dot(&self, mut path: String) -> Result<(), Error> {
        if !path.contains(".dot") {
            path = path + ".dot";
        }

        let mut file: File = File::create(path)?;

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

    fn export_directed_bfs_to_dot(&self, start: Node, mut path: String) -> Result<(), Error>
    where
        Self: Sized,
    {
        if !path.contains(".dot") {
            path = path + ".dot";
        }

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

    fn export_directed_dfs_to_dot(&self, start: Node, mut path: String) -> Result<(), Error>
    where
        Self: Sized,
        Node: Copy + Eq + Hash + Display + From<usize>,
    {
        if !path.contains(".dot") {
            path = path + ".dot";
        }
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
                Edge::Forward(parent, node) => {
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
    fn export_undirected_to_dot(&self, mut path: String) -> Result<(), Error>
    where
        Self: Sized + UndirectedGraph<Node>,
    {
        if !path.contains(".dot") {
            path = path + ".dot";
        }

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

    fn import_undirected_from_file(path: String) -> Result<Self, Error>
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
                let n: usize = content.parse().map_err(|_| {
                    Error::new(
                        ErrorKind::InvalidData,
                        format!("Line 1 from file is invalid"),
                    )
                })?;

                for i in 0..n {
                    graph.add_node(Node::from(i));
                }
            } else {
                let values: Vec<usize> = content
                    .split(',')
                    .map(|x| {
                        x.trim().parse::<usize>().map(|v| v - 1).map_err(|_| {
                            Error::new(
                                ErrorKind::InvalidData,
                                format!("Invalid node was found during file creation: {} ", x),
                            )
                        })
                    })
                    .collect::<Result<Vec<_>, Error>>()?;

                graph.add_undirected_edge(Node::from(values[0]), Node::from(values[1]));
            }
        }

        Ok(graph)
    }

    fn export_undirected_dfs_to_dot(&self, start: Node, mut path: String) -> Result<(), Error>
    where
        Self: Sized + UndirectedGraph<Node>,
        Node: Copy + Eq + Hash + Display + From<usize> + PartialOrd,
    {
        if Node::from(self.nodes().count()) < start {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Node {} isn't present in graph!", start),
            ));
        }

        if !path.contains(".dot") {
            path = path + ".dot";
        }

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
                        if !visited_edges.contains(&(node, parent))
                            && !visited_edges.contains(&(parent, node))
                        {
                            writeln!(file, " {} -- {} ", parent, node)?;
                            visited_edges.insert((node, parent));
                        }
                    }
                }
                DfsEvent::NonTreeEdge(node, parent) => {
                    if !visited_edges.contains(&(node, parent))
                        && !visited_edges.contains(&(parent, node))
                    {
                        writeln!(file, " {} -- {} [style=dashed];", node, parent)?;
                        visited_edges.insert((node, parent));
                    }
                }
                DfsEvent::Finish(_) => continue,
            }
        }

        writeln!(file, " }}")?;

        Ok(())
    }

    fn export_undirected_bfs_to_dot(&self, start: Node, mut path: String) -> Result<(), Error>
    where
        Self: Sized + UndirectedGraph<Node>,
        Node: Copy + Eq + Hash + Display + From<usize> + PartialOrd,
    {
        if Node::from(self.nodes().count()) < start {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Node {} isn't present in graph!", start),
            ));
        }

        if !path.contains(".dot") {
            path = path + ".dot";
        }

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
