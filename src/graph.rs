use std::collections::{HashMap, HashSet, VecDeque};
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

    fn dfs_edges<'a>(&'a self, start_nodes: &[&'a Node]) -> DfsEdgesIter<'a, Node, Self>
    where
        Self: Sized,
    {
        DfsEdgesIter::new(self, start_nodes)
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

pub struct DfsEdgesIter<'a, Node, G> {
    graph: &'a G,
    visited: HashSet<&'a Node>,
    discovery: HashMap<&'a Node, usize>,
    finish: HashMap<&'a Node, usize>,
    time: usize,
    stack_hash: HashSet<&'a Node>,
    pending_edges: VecDeque<Edge<&'a Node>>,
}

impl<'a, Node: Eq + Hash, G: Graph<Node>> DfsEdgesIter<'a, Node, G> {
    /// Performs a depth-first search (DFS) starting from the given nodes,
    /// recording discovery and finish times for each visited node.
    pub fn times(
        &mut self,
        start_nodes: &[&'a Node],
    ) -> (&HashMap<&'a Node, usize>, &HashMap<&'a Node, usize>) {
        for n in start_nodes {
            self.dfs(n);
        }
        (&self.discovery, &self.finish)
    }

    fn new(graph: &'a G, start_nodes: &[&'a Node]) -> Self {
        let mut iter = Self {
            graph,
            visited: HashSet::with_capacity(graph.order()),
            discovery: HashMap::with_capacity(graph.order()),
            finish: HashMap::with_capacity(graph.order()),
            time: 0,
            stack_hash: HashSet::with_capacity(graph.order()),
            pending_edges: VecDeque::with_capacity(graph.size()),
        };

        for n in start_nodes {
            iter.dfs(n);
        }
        iter
    }

    fn dfs(&mut self, start: &'a Node) {
        self.visited.insert(start);
        self.discovery.insert(start, self.time);
        self.time += 1;
        self.stack_hash.insert(start);

        for neighbor in self.graph.neighbors(start) {
            if !self.visited.contains(neighbor) {
                self.pending_edges.push_back(Edge::Tree((start, neighbor)));
                self.dfs(neighbor);
            } else if self.stack_hash.contains(neighbor) {
                self.pending_edges.push_back(Edge::Back((start, neighbor)));
            } else if self.discovery[start] < self.discovery[neighbor] {
                self.pending_edges
                    .push_back(Edge::Foward((start, neighbor)));
            } else {
                self.pending_edges.push_back(Edge::Cross((start, neighbor)));
            }
        }

        self.stack_hash.remove(start);
        self.finish.insert(start, self.time);
        self.time += 1;
    }
}

impl<'a, Node: Eq + Hash, G: Graph<Node>> Iterator for DfsEdgesIter<'a, Node, G> {
    type Item = Edge<&'a Node>;
    fn next(&mut self) -> Option<Self::Item> {
        self.pending_edges.pop_front()
    }
}

#[allow(dead_code)]
pub trait Digraph<Node: Copy + Eq + Hash>: Graph<Node> {}
