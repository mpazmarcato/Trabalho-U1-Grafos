use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

pub enum Edge<Node> {
    Tree((Node, Node)),
    Back((Node, Node)),
    Foward((Node, Node)),
    Cross((Node, Node)),
}

pub trait Graph<Node: Eq + Hash> {
    fn order(&self) -> usize;
    fn size(&self) -> usize;

    fn add_node(&mut self, n: Node);
    fn remove_node(&mut self, n: &Node);

    fn add_edge(&mut self, n: Node, m: Node);
    fn remove_edge(&mut self, n: &Node, m: &Node);

    type Neighbors<'a>: Iterator<Item = &'a Node>
    where
        Self: 'a,
        Node: 'a;
    fn neighbors<'a>(&'a self, n: &Node) -> Self::Neighbors<'a>;

    // HACK: Maybe we could implement the remaining ones at a trait level 🔥🔥🔥.
    fn connected(&self) -> bool;
    fn biconnected_components(&self) -> &[Vec<&Node>];

    fn biparted(&self) -> bool;

    fn has_edge(&self, n: &Node, m: &Node) -> bool {
        self.neighbors(n).any(|neighbor| *neighbor == *m)
    }

    fn node_degree(&self, n: &Node) -> usize {
        self.neighbors(n).count()
    }

    fn dfs<'a>(&'a self, start: &'a Node) -> DfsIter<'a, Node, Self>
    where
        Self: Sized,
    {
        DfsIter::new(self, start)
    }

    fn bfs<'a>(&'a self, start: &'a Node) -> BfsIter<'a, Node, Self>
    where
        Self: Sized,
    {
        BfsIter::new(self, start)
    }

    fn back_edges<'a>(&'a self, _start: &'a Node) -> Vec<Edge<&'a Node>> {
        todo!()
    }
}

pub struct DfsIter<'a, Node, G> {
    graph: &'a G,
    stack: Vec<&'a Node>,
    visited: HashSet<&'a Node>,
}

impl<'a, Node: Eq + Hash, G: Graph<Node>> DfsIter<'a, Node, G> {
    fn new(graph: &'a G, start: &'a Node) -> Self {
        let mut visited = HashSet::with_capacity(graph.order());
        visited.insert(start);
        Self {
            graph,
            stack: vec![start],
            visited,
        }
    }
}

impl<'a, Node: Eq + Hash, G: Graph<Node>> Iterator for DfsIter<'a, Node, G> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        for neighbor in self.graph.neighbors(node) {
            if self.visited.insert(neighbor) {
                self.stack.push(neighbor);
            }
        }
        Some(node)
    }
}

pub struct BfsIter<'a, Node, G> {
    graph: &'a G,
    queue: VecDeque<&'a Node>,
    visited: HashSet<&'a Node>,
}

impl<'a, Node: Eq + Hash, G: Graph<Node>> BfsIter<'a, Node, G> {
    fn new(graph: &'a G, start: &'a Node) -> Self {
        let mut visited = HashSet::with_capacity(graph.order());
        visited.insert(start);
        Self {
            graph,
            queue: VecDeque::from(vec![start]),
            visited,
        }
    }
}

impl<'a, Node: Eq + Hash, G: Graph<Node>> Iterator for BfsIter<'a, Node, G> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        _ = self.graph;
        _ = self.queue;
        _ = self.visited;
        // Tip: The iterator should return Some(next item) or None in case there's no item.
        todo!("BFS iterator")
    }
}

// TODO: Implement necessary functions for digraph
#[allow(dead_code)]
pub trait Digraph<Node: Copy + Eq + Hash>: Graph<Node> {}
