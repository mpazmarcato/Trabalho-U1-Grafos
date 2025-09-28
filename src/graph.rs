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
    fn connected(&self) -> bool;

    fn biconnected_components(&self) -> &[Vec<Node>];

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

/// Represents an event that occurs during a depth-first search (DFS) traversal.
///
/// This enum is used to describe the different types of events that can be
/// encountered while performing DFS on a graph. It is generic over the `Node`
/// type, which represents the nodes in the graph.
///
/// # Variants
/// - `Discover(Node, Option<Node>)`: Indicates that a node has been discovered for the first time.
///   The `Option<Node>` represents the parent node in the DFS tree (`None` for the start node).
/// - `Finish(Node)`: Indicates that all descendants of a node have been visited and the node is finished.
/// - `NonTreeEdge(Node, Node)`: Indicates that a non-tree edge (back, forward, or cross edge) is found
///   from the first node to the second node.
#[derive(Debug)]
pub enum DfsEvent<Node> {
    Discover(Node, Option<Node>),
    Finish(Node),
    NonTreeEdge(Node, Node),
}

/// Represents a iterator over a depth-first-search (DFS) traversal.
///
/// The iteration yields a `DfsEvent<Node>` over each instasse of `next`.
///
/// # Examples
///
/// ```rust,ignore
/// // Should print every event that happens
/// // when you traverse the graph starting from node `u`.
/// let mut iter = graph.dfs(u);
/// for e in iter {
///     println!("{:?}", e);
/// }
///
/// // Should print every event that's a node discovery.
/// let mut iter = graph.dfs(0);
/// for e in iter.filter(|e| matches!(e, DfsEvent::Discover(_))) {
///     println!("{:?}", e);
/// }
/// ```
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

    /// Sets the `start_node` field of a `DfsIter` manually.
    ///
    /// This enables starting another DFS while maintaing the inner parts of the iterator
    /// initialized, like the `visited` dictionary.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // Initializes a iterator to search starting from node `u`.
    /// let mut iter = graph.dfs(u);
    /// for e in &mut iter {
    ///     println!("{e}");
    /// }
    ///
    /// // Does the same for `v`.
    /// iter.new_start(v);
    /// for e in &mut iter {
    ///     println!("{e}");
    /// }
    /// ```
    pub fn new_start(&mut self, start: Node) {
        self.start_node = Some(start)
    }
}

impl<'a, Node: Eq + Hash + Copy, G: Graph<Node>> Iterator for DfsIter<'a, Node, G> {
    type Item = DfsEvent<Node>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(start_node) = self.start_node.take()
            && self.visited.insert(start_node)
        {
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

/// Represents the classification of an edge in a graph during a depth-first search (DFS).
///
/// This enum is used to categorize edges based on the DFS traversal. It is generic
/// over the `Node` type, which represents the nodes in the graph. The classification
/// is based on the relationship between the two nodes connected by the edge in the DFS tree.
///
/// # Variants
/// - `Tree(u, v)`: An edge from a parent `u` to a child `v` in the DFS tree.
/// - `Back(u, v)`: An edge from a node `u` to its ancestor `v` in the DFS tree. This indicates a cycle.
/// - `ParentBack(u, v)`: A special case of a back edge where `v` is the direct parent of `u`.
///   This is common in undirected graphs.
/// - `Foward(u, v)`: An edge from a node `u` to its descendant `v` that is not a tree edge.
/// - `Cross(u, v)`: An edge between two nodes `u` and `v` such that neither is an ancestor of the other.
#[derive(Debug)]
pub enum Edge<Node> {
    Tree(Node, Node),
    Back(Node, Node),
    ParentBack(Node, Node),
    Foward(Node, Node),
    Cross(Node, Node),
}

/// An iterator that performs a depth-first search (DFS) and classifies the edges of the graph.
///
/// This iterator wraps a `DfsIter` and uses its events to classify each edge of the
/// graph into one of the categories defined by the `Edge` enum. It yields an `Edge<Node>`
/// for each edge encountered during the traversal.
///
/// # Examples
///
/// ```rust
/// use graphs_algorithms::{Edge, Graph};
/// use graphs_algorithms::graphs::AdjacencyList;
///
/// let mut graph = AdjacencyList::default();
/// graph.add_node(0);
/// graph.add_node(1);
/// graph.add_node(2);
/// graph.add_edge(0, 1);
/// graph.add_edge(1, 2);
/// graph.add_edge(2, 0); // Cycle
///
/// let mut edges_iter = graph.classify_edges(0);
///
/// assert!(matches!(edges_iter.next(), Some(Edge::Tree(0, 1))));
/// assert!(matches!(edges_iter.next(), Some(Edge::Tree(1, 2))));
/// assert!(matches!(edges_iter.next(), Some(Edge::Back(2, 0))));
/// assert!(edges_iter.next().is_none());
/// ```
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

    /// Sets the `start_node` field of the inner `DfsIter` manually.
    ///
    /// This enables classifying edges from another components of a graph.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use graphs_algorithms::graphs::AdjacencyList;
    /// use graphs_algorithms::{Edge, Graph};
    ///
    /// let mut graph = AdjacencyList::default();
    /// graph.add_node(0);
    /// graph.add_node(1);
    /// graph.add_node(2);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(2, 0);
    ///
    /// let mut iter = graph.classify_edges(0);
    /// assert!(matches!(iter.next(), Some(Edge::Tree(0, 1))));
    /// assert!(iter.next().is_none());
    ///
    /// iter.new_start(2);
    /// assert!(matches!(iter.next(), Some(Edge::Cross(2, 0))));
    /// assert!(iter.next().is_none());
    /// ```
    pub fn new_start(&mut self, start: Node) {
        self.iter.new_start(start);
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
