use std::collections::{HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

pub trait Graph<Node: Eq + Hash + Copy> {
    fn order(&self) -> usize;
    fn size(&self) -> usize;

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

impl<'a, Node: Eq + Hash + Copy + Debug, G: Graph<Node>> Iterator for DfsIter<'a, Node, G> {
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

#[cfg(test)]
mod test {
    use crate::{DfsEvent, Graph, graphs::AdjacencyList};

    #[test]
    fn test_dfs_with_cycle() {
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
    fn test_dfs_simple_path() {
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
