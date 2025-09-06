// HACK: Maybe make graph generic over node types?
// FIXME: Remove this macro eventually.
pub trait Graph {
    fn nodes(&self) -> usize;
    fn edges(&self) -> usize;
    fn add_node(&mut self, n: usize);
    fn remove_node(&mut self, n: usize);
    fn add_edge(&mut self, n: usize, m: usize);
    fn remove_edge(&mut self, n: usize, m: usize);
    fn neighbors(&self, n: usize) -> Vec<usize>;
    fn has_edge(&self, n:usize, m:usize) -> bool;
    // TODO: add more methods like, DFS, BFS, etc.
}
