pub trait Graph<Node: Copy + Eq + std::hash::Hash> {
    fn nodes(&self) -> usize;
    fn edges(&self) -> usize;
    fn add_node(&mut self, n: Node);
    fn remove_node(&mut self, n: Node);
    fn add_edge(&mut self, n: Node, m: Node);
    fn remove_edge(&mut self, n: Node, m: Node);
    fn neighbors(&self, n: Node) -> Vec<Node>;
    fn has_edge(&self, n: Node, m: Node) -> bool;
    fn dfs(&self, start: Node) -> usize;
    fn bfs(&self, start: Node) -> usize;
    fn node_degree(&self, n: Node) -> Option<usize>;
    fn connected(&self) -> bool;
    fn biparted(&self) -> bool;
    fn biconnected_components(&self) -> Vec<impl Graph<Node>>;
}

#[allow(dead_code)]
pub trait Digraph<Node: Copy + Eq + std::hash::Hash>: Graph<Node> {
    // fn bfs(&self, start: Node) -> usize;
}
