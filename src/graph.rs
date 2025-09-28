use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

pub trait Graph<Node: Eq + Hash + Copy> {
    fn order(&self) -> usize;
    fn size(&self) -> usize;
    fn underlying_graph(&self) -> Self;

    fn add_node(&mut self, n: Node);
    fn remove_node(&mut self, n: Node);

    fn add_edge(&mut self, n: Node, m: Node);
    fn remove_edge(&mut self, n: Node, m: Node);

    type Neighbors<'a>: Iterator<Item = Node>
    where
        Self: 'a,
        Node: 'a;
    fn neighbors<'a>(&'a self, n: Node) -> Self::Neighbors<'a>;

    fn connected(&self) -> bool;
    fn biconnected_components(&self) -> &[Vec<Node>];

    fn biparted(&self) -> bool;

    fn has_edge(&self, n: Node, m: Node) -> bool {
        self.neighbors(n).any(|neighbor| neighbor == m)
    }

    fn node_degree(&self, n: Node) -> usize {
        self.neighbors(n).count()
    }

    fn dfs(&self, start: Node) -> DfsIter<'_, Node, Self>
    where
        Self: Sized,
    {
        DfsIter::new(self, start)
    }

    fn bfs(&self, start: Node) -> BfsIter<'_, Node, Self>
    where
        Self: Sized,
    {
        BfsIter::new(self, start)
    }

    fn classify_edges(&self, start: Node) -> DfsEdgesIter<'_, Node, Self>
    where
        Self: Sized,
    {
        DfsEdgesIter::new(self, start)
    }
}

pub trait UndirectedGraph<Node: Copy + Eq + Hash>: Graph<Node> {
    fn add_undirected_edge(&mut self, n: Node, m: Node) {
        self.add_edge(n, m);
        self.add_edge(m, n);
    }

    fn remove_undirected_edge(&mut self, n: Node, m: Node) {
        self.remove_edge(n, m);
        self.remove_edge(m, n);
    }

    fn classify_undirected_edges<'a>(&'a self, start: Node) -> impl Iterator<Item = Edge<Node>>
    where
        Self: Sized,
        Node: 'a,
    {
        DfsEdgesIter::new(self, start)
            .filter(|edge| matches!(edge, Edge::Tree(_, _) | Edge::Back(_, _)))
    }
}

pub struct DfsIter<'a, Node, G>
where
    G: Graph<Node>,
    Node: Eq + Hash + Copy,
    Self: 'a,
{
    graph: &'a G,
    stack: Vec<(Node, G::Neighbors<'a>)>,
    visited: HashSet<Node>,
    start_node: Option<Node>,
}

impl<'a, Node: Eq + Hash + Copy, G: Graph<Node>> DfsIter<'a, Node, G> {
    fn new(graph: &'a G, start: Node) -> Self {
        Self {
            graph,
            stack: vec![],
            visited: HashSet::with_capacity(graph.order()),
            start_node: Some(start),
        }
    }

    fn _new_start(&mut self, start: Node) {
        self.start_node = Some(start)
    }
}

#[derive(Debug)]
pub enum DfsEvent<Node> {
    /// A node is discovered for the first time.
    /// The Option<Node> is the parent in the DFS tree (None for the start Node).
    Discover(Node, Option<Node>),
    /// All descendants of a node have been visited.
    Finish(Node),
    /// A non-tree edge is found from the first node to the second.
    NonTreeEdge(Node, Node),
}

impl<'a, Node: Eq + Hash + Copy, G: Graph<Node>> Iterator for DfsIter<'a, Node, G> {
    type Item = DfsEvent<Node>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(start_node) = self.start_node.take() {
            self.visited.insert(start_node);
            self.stack
                .push((start_node, self.graph.neighbors(start_node)));
            return Some(DfsEvent::Discover(start_node, None));
        }

        if let Some((node, mut neighbors)) = self.stack.pop() {
            if let Some(neighbor) = neighbors.next() {
                self.stack.push((node, neighbors));

                if self.visited.insert(neighbor) {
                    self.stack.push((neighbor, self.graph.neighbors(neighbor)));
                    return Some(DfsEvent::Discover(neighbor, Some(node)));
                } else {
                    return Some(DfsEvent::NonTreeEdge(node, neighbor));
                }
            } else {
                return Some(DfsEvent::Finish(node));
            }
        }

        None
    }
}

pub struct BfsIter<'a, Node, G> {
    graph: &'a G,
    queue: VecDeque<Node>,
    visited: HashSet<Node>,
}

impl<'a, Node: Eq + Hash + Copy, G: Graph<Node>> BfsIter<'a, Node, G> {
    fn new(graph: &'a G, start: Node) -> Self {
        let mut visited = HashSet::with_capacity(graph.order());
        visited.insert(start);
        Self {
            graph,
            queue: VecDeque::from(vec![start]),
            visited,
        }
    }
}

impl<'a, Node: Eq + Hash + Copy, G: Graph<Node>> Iterator for BfsIter<'a, Node, G> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.queue.pop_front()?;
        for neighbor in self.graph.neighbors(node) {
            if self.visited.insert(neighbor) {
                self.queue.push_back(neighbor);
            }
        }
        Some(node)
    }
}

#[derive(Debug)]
pub enum Edge<Node> {
    Tree(Node, Node),
    Back(Node, Node),
    ParentBack(Node, Node),
    Foward(Node, Node),
    Cross(Node, Node),
}

pub struct DfsEdgesIter<'a, Node, G>
where
    G: Graph<Node>,
    Node: Eq + Hash + Copy,
    Self: 'a,
{
    iter: DfsIter<'a, Node, G>,
    time: usize,
    discovery: HashMap<Node, usize>,
    finish: HashMap<Node, usize>,
    parent: HashMap<Node, Node>,
    stack_hash: HashSet<Node>,
}

impl<'a, Node: Eq + Hash + Copy, G: Graph<Node>> DfsEdgesIter<'a, Node, G> {
    fn new(graph: &'a G, start: Node) -> Self {
        Self {
            iter: DfsIter::new(graph, start),
            time: 0,
            discovery: HashMap::with_capacity(graph.order()),
            finish: HashMap::with_capacity(graph.order()),
            parent: HashMap::with_capacity(graph.order()),
            stack_hash: HashSet::with_capacity(graph.order()),
        }
    }
}

impl<'a, Node: Eq + Hash + Copy, G: Graph<Node>> Iterator for DfsEdgesIter<'a, Node, G> {
    type Item = Edge<Node>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(event) = self.iter.next() {
                match event {
                    DfsEvent::Discover(node, maybe_parent) => {
                        self.stack_hash.insert(node);
                        self.discovery.insert(node, self.time);
                        self.time += 1;
                        if let Some(parent) = maybe_parent {
                            self.parent.insert(node, parent);
                            return Some(Edge::Tree(parent, node));
                        }
                    }
                    DfsEvent::Finish(node) => {
                        self.stack_hash.remove(&node);
                        self.finish.insert(node, self.time);
                        self.time += 1;
                    }
                    DfsEvent::NonTreeEdge(node, neighbor) => {
                        if self.stack_hash.contains(&neighbor) {
                            if self
                                .parent
                                .get(&node)
                                .is_some_and(|&parent| parent == neighbor)
                            {
                                return Some(Edge::ParentBack(node, neighbor));
                            } else {
                                return Some(Edge::Back(node, neighbor));
                            }
                        } else if self.discovery.get(&node).is_some_and(|t1| {
                            self.discovery.get(&neighbor).is_some_and(|t2| t1 < t2)
                        }) {
                            return Some(Edge::Foward(node, neighbor));
                        } else {
                            return Some(Edge::Cross(node, neighbor));
                        }
                    }
                }
            } else {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{DfsEvent, Graph, graphs::AdjacencyList};

    #[test]
    fn dfs_with_cycle() {
        let mut g = AdjacencyList::default();
        g.add_node(0);
        g.add_node(1);
        g.add_node(2);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 0); // Cycle

        let mut dfs = g.dfs(0);

        assert!(matches!(dfs.next(), Some(DfsEvent::Discover(0, None))));
        assert!(matches!(dfs.next(), Some(DfsEvent::Discover(1, Some(0)))));
        assert!(matches!(dfs.next(), Some(DfsEvent::Discover(2, Some(1)))));
        assert!(matches!(dfs.next(), Some(DfsEvent::NonTreeEdge(2, 0))));
        assert!(matches!(dfs.next(), Some(DfsEvent::Finish(2))));
        assert!(matches!(dfs.next(), Some(DfsEvent::Finish(1))));
        assert!(matches!(dfs.next(), Some(DfsEvent::Finish(0))));
        assert!(dfs.next().is_none());
    }

    #[test]
    fn dfs_simple_path() {
        let mut g = AdjacencyList::default();
        g.add_node(0);
        g.add_node(1);
        g.add_node(2);
        g.add_edge(0, 1);
        g.add_edge(1, 2);

        let mut dfs = g.dfs(0);

        assert!(matches!(dfs.next(), Some(DfsEvent::Discover(0, None))));
        assert!(matches!(dfs.next(), Some(DfsEvent::Discover(1, Some(0)))));
        assert!(matches!(dfs.next(), Some(DfsEvent::Discover(2, Some(1)))));
        assert!(matches!(dfs.next(), Some(DfsEvent::Finish(2))));
        assert!(matches!(dfs.next(), Some(DfsEvent::Finish(1))));
        assert!(matches!(dfs.next(), Some(DfsEvent::Finish(0))));
        assert!(dfs.next().is_none());
    }

    #[test]
    fn single_node_dfs() {
        let mut g = AdjacencyList::default();
        g.add_node(0);

        let mut dfs = g.dfs(0);

        assert!(matches!(dfs.next(), Some(DfsEvent::Discover(0, None))));
        assert!(matches!(dfs.next(), Some(DfsEvent::Finish(0))));
        assert!(dfs.next().is_none());
    }
}
