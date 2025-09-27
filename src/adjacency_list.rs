use crate::Graph;
use crate::graph::UndirectedGraph;
use crate::graphs::{AdjacencyMatrix, IncidenceMatrix};

#[derive(Debug, Clone)]
pub struct AdjacencyList(pub Vec<Vec<usize>>);

impl AdjacencyList {
    pub fn from_adjacency_matrix(matrix: &AdjacencyMatrix) -> Self {
        let mut adjacency_list = vec![Vec::new(); matrix.0.len()];

        for (i, row) in matrix.0.iter().enumerate() {
            adjacency_list[i].extend(
                row.iter()
                    .enumerate()
                    .filter_map(|(j, &val)| (val != 0).then_some(j)),
            );
        }
        AdjacencyList(adjacency_list)
    }

    pub fn from_incidence_matrix(_matrix: &IncidenceMatrix) -> Self {
        todo!()
    }

    pub fn bfs(&self) -> i32 {
        self.bfs_from_node(0)
    }

    pub fn bfs_from_node(&self, start: usize) -> i32 {
        if start >= self.0.len() {
            return 0; // nó inválido
        }

        let mut visited = vec![false; self.0.len()];
        let mut queue = std::collections::VecDeque::new();
        let mut count = 0;

        visited[start] = true;
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            count += 1;

            for &neighbor in &self.0[node] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }

        count
    }
}

impl Graph<usize> for AdjacencyList {
    fn order(&self) -> usize {
        self.0.len()
    }

    // TODO: fix the duplication
    fn size(&self) -> usize {
        self.0.iter().map(|neighbors| neighbors.len()).sum()
    }

    fn add_node(&mut self, _n: usize) {
        self.0.push(Vec::new());
    }

    fn remove_node(&mut self, n: usize) {
        if n < self.0.len() {
            self.0.remove(n);
            for neighbors in self.0.iter_mut() {
                neighbors.retain(|&x| x != n);
                for x in neighbors.iter_mut() {
                    if *x > n {
                        *x -= 1;
                    }
                }
            }
        }
    }

    fn add_edge(&mut self, n: usize, m: usize) {
        if self.0.get(m).is_some()
            && let Some(n_edges) = self.0.get_mut(n)
            && !n_edges.contains(&m)
        {
            n_edges.push(m);
        }
    }

    fn remove_edge(&mut self, n: usize, m: usize) {
        if let Some(edges) = self.0.get_mut(n) {
            edges.retain(|&x| x != m);
        }
    }

    type Neighbors<'a> = Box<dyn Iterator<Item = usize> + 'a>;

    fn neighbors<'a>(&'a self, n: usize) -> Self::Neighbors<'a> {
        match self.0.get(n) {
            Some(edges) => Box::new(edges.iter().copied()),
            None => Box::new(std::iter::empty()),
        }
    }

    fn connected(&self) -> bool {
        for i in 0..self.order() {
            if self.dfs(i).count() != self.order() {
                return false;
            }
        }
        true
    }

    fn biparted(&self) -> bool {
        todo!()
    }

    #[allow(unreachable_code)]
    fn biconnected_components(&self) -> &[Vec<usize>] {
        todo!();
    }
}

impl UndirectedGraph<usize> for AdjacencyList {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn add_new_node() {
        // Graph:
        //     0       1
        //       \   /   \
        //         3      2
        //       /
        //      4
        let mut list = AdjacencyList(vec![vec![3], vec![2, 3], vec![1], vec![0, 1, 4], vec![3]]);
        list.add_node(5);

        // Current graph:
        //     0       1
        //       \   /   \
        //         3      2
        //       /
        //      4    5
        assert!(list.order() == 6);
    }

    #[test]
    fn add_new_node_and_edge() {
        // Graph:
        //     0       1
        //       \   /   \
        //         3      2
        //       /
        //      4
        let mut list = AdjacencyList(vec![vec![3], vec![2, 3], vec![1], vec![0, 1, 4], vec![3]]);
        list.add_node(5);
        list.add_edge(3, 5);

        // Current graph:
        //     0       1
        //       \   /   \
        //         3      2
        //       /   \
        //      4     5
        assert!(list.order() == 6);
        // assert!(list.size() == 5);
        assert!(list.has_edge(3, 5));
    }

    #[test]
    fn add_new_node_and_loop_edge() {
        // Graph:
        //     0       1
        //       \   /   \
        //         3      2
        //       /   \
        //      4     5
        let mut list = AdjacencyList(vec![vec![3], vec![2, 3], vec![1], vec![0, 1, 4], vec![3]]);
        list.add_node(5);
        list.add_edge(3, 5);
        list.add_edge(2, 2);

        // Current graph:
        //     0       1      -
        //       \   /   \  /   \
        //         3      2      |
        //       /   \      \    /
        //      4     5       -
        assert!(list.order() == 6);
        // assert!(list.size() == 6); // FIXME: uncomment this later when size() is corrected
        assert!(list.has_edge(3, 5));
        assert!(list.has_edge(2, 2));
    }

    #[test]
    fn connected_graph() {
        // Graph: 2 ── 0 ── 1
        // should be connected.
        assert!(AdjacencyList(vec![vec![1, 2], vec![0], vec![0]]).connected())
    }

    #[test]
    fn unconnected_graph() {
        // Graph: 2    0 ── 1
        // should be not connected.
        assert!(!AdjacencyList(vec![vec![1], vec![0], vec![]]).connected())
    }
}
