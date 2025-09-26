use crate::Graph;
use crate::graph::{DfsEvent, UndirectedGraph};
use crate::graphs::{AdjacencyMatrix, IncidenceMatrix};

#[derive(Debug, Clone, Default)]
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

    // FIXME: fix the duplication!! Only working for digraphs.
    fn size(&self) -> usize {
        self.0.iter().map(|neighbors| neighbors.len()).sum()
    }

    fn underlying_graph(&self) -> Self {
        let mut list = AdjacencyList(vec![Vec::new(); self.0.len()]);

        for (idx_r, row) in self.0.iter().enumerate() {
            for &col in row.iter() {
                if !list.has_edge(idx_r, col) {
                    list.add_undirected_edge(idx_r, col);
                }
            }
        }
        list
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

    // TODO: Only working for undirected graphs...
    fn connected(&self) -> bool {
        for i in 0..self.order() {
            if self
                .dfs(i)
                .filter(|event| matches!(event, DfsEvent::Discover(_, _)))
                .count()
                != self.order()
            {
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
    fn connected_undirected_graph() {
        // Graph: 2 ── 0 ── 1
        // should be connected.
        assert!(AdjacencyList(vec![vec![1, 2], vec![0], vec![0]]).connected())
    }

    #[test]
    fn unconnected_undirected_graph() {
        // Graph: 2    0 ── 1
        // should be not connected.
        assert!(!AdjacencyList(vec![vec![1], vec![0], vec![]]).connected())
    }

    #[test]
    fn underlying_graph_conversion() {
        // Graph:
        //     0      -> 1
        //       \    /   \
        //        -> 3     -> 2
        //       /
        //      4
        let original_list = AdjacencyList(vec![vec![3], vec![2], vec![], vec![1], vec![3]]);

        let underlying_list = original_list.underlying_graph();

        // Current graph:
        //     0        - 1
        //       \    /    \
        //        - 3       - 2
        //       /
        //      4
        assert_eq!(original_list.order(), underlying_list.order());
        // assert_eq!(original_list.size(), underlying_list.size()); // FIXME: uncomment when size duplication is fixed!
        assert!(underlying_list.connected());
    }

    #[test]
    fn underlying_graph_conversion_and_node_delete_after() {
        // Graph:
        // 0 -> 1 -> 2 <- 3
        //      \    ^
        //       \   |
        //       ->  4
        let original_list = AdjacencyList(vec![vec![1], vec![2, 4], vec![], vec![2], vec![2]]);

        let mut underlying_list = original_list.underlying_graph();

        // Current graph:
        // 0 -- 1 -- 2 -- 3
        //      \    |
        //       \   |
        //        -  4
        assert_eq!(original_list.order(), underlying_list.order());
        // assert_eq!(original_list.size(), underlying_list.size()); // FIXME: uncomment when size duplication is fixed!
        assert!(underlying_list.connected());

        underlying_list.remove_node(2);
        // Current graph:
        // 0 -- 1         2
        //      \
        //       \
        //        -  3
        assert_ne!(original_list.order(), underlying_list.order());
        assert!(!underlying_list.connected());
    }

    #[test]
    fn graph_add_new_node() {
        // Graph:
        //     0      -> 1
        //       \    /   \
        //        -> 3     -> 2
        //       /
        //      4
        let mut list = AdjacencyList(vec![vec![3], vec![2], vec![], vec![1], vec![3]]);
        list.add_node(5);

        // Current graph:
        //     0      -> 1
        //       \    /   \
        //        -> 3     -> 2
        //       /
        //      4    5
        assert!(list.order() == 6);
        assert!(list.size() == 4);
        assert!(!list.connected());
    }

    #[test]
    fn graph_add_new_node_and_edge() {
        // Graph:
        //     0      -> 1
        //       \    /   \
        //        -> 3     -> 2
        //       /
        //      4
        let mut list = AdjacencyList(vec![vec![3], vec![2], vec![], vec![1], vec![3]]);
        list.add_node(5);
        list.add_edge(3, 5);

        // Current graph:
        //     0      -> 1
        //       \    /   \
        //        -> 3     -> 2
        //       /    \
        //      4      -> 5
        assert!(list.order() == 6);
        assert!(list.size() == 5);
        assert!(list.has_edge(3, 5));
        // assert!(list.connected()); // FIXME: implementar uma conversão de digrafo para seu subjacente para que funcione
    }

    #[test]
    fn graph_add_new_node_and_loop_edge() {
        // Graph:
        //     0      -> 1
        //       \    /   \
        //        -> 3     -> 2
        //       /
        //      4
        let mut list = AdjacencyList(vec![vec![3], vec![2], vec![], vec![1], vec![3]]);
        list.add_node(5);
        list.add_edge(3, 5);
        list.add_edge(2, 2);

        // Current graph:
        //     0      -> 1      ---
        //       \    /   \    /   \
        //        -> 3     -> 2    |
        //       /    \        ^   /
        //      4      -> 5    \--
        assert!(list.order() == 6);
        assert!(list.size() == 6);
        assert!(list.has_edge(3, 5));
        assert!(list.has_edge(2, 2));
    }

    #[test]
    fn graph_remove_edge() {
        // Graph:
        //     0      -> 1
        //       \    /   \
        //        -> 3     -> 2
        //       /    \
        //      4      -> 5
        let mut list = AdjacencyList(vec![vec![3], vec![2], vec![], vec![1, 5], vec![3], vec![]]);

        list.remove_edge(0, 3);
        // Current graph:
        //     0       -> 1
        //           /   \
        //          3     -> 2
        //       /    \
        //      4      -> 5
        assert!(list.size() == 4);
        assert!(!list.has_edge(0, 3));
        assert!(list.has_edge(4, 3));
        assert!(!list.connected());
    }

    #[test]
    fn graph_remove_edge_loop() {
        // Graph:
        //     0      -> 1      ---
        //       \    /   \    /   \
        //        -> 3     -> 2    |
        //       /    \        ^   /
        //      4      -> 5    \--
        let mut list = AdjacencyList(vec![vec![3], vec![2], vec![2], vec![1, 5], vec![3], vec![]]);
        list.remove_edge(2, 2);

        // Current graph:
        //     0      -> 1
        //       \    /   \
        //        -> 3     -> 2
        //       /    \
        //      4      -> 5
        assert!(list.size() == 5);
        assert!(!list.has_edge(2, 2));
        assert!(list.has_edge(1, 2));
        // assert!(list.connected()); // FIXME: implementar uma conversão de digrafo para seu subjacente para que funcione
    }

    #[test]
    fn graph_remove_node() {
        // Graph:
        //     0      -> 1
        //       \    /   \
        //        -> 3     -> 2
        //       /    \
        //      4      -> 5
        let mut list = AdjacencyList(vec![vec![3], vec![2], vec![], vec![1, 5], vec![3], vec![]]);
        list.remove_node(3);

        // Current graph:
        //     0      1
        //              \
        //               -> 2
        //
        //      3       4
        assert!(list.order() == 5);
        assert!(list.size() == 1);
        assert!(!list.has_edge(0, 3));
        assert!(list.has_edge(1, 2));
        assert!(!list.connected());
    }

    #[test]
    fn undirected_graph_add_new_node() {
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
        //      4     5
        assert!(list.order() == 6);
        // assert!(list.size() == 4);// TODO: uncomment when fixed
        assert!(!list.connected());
    }

    #[test]
    fn undirected_graph_add_new_node_and_edge() {
        // Graph:
        //     0       1
        //       \   /   \
        //         3      2
        //       /
        //      4
        let mut list = AdjacencyList(vec![vec![3], vec![2, 3], vec![1], vec![0, 1, 4], vec![3]]);
        list.add_node(5);
        list.add_undirected_edge(3, 5);

        // Current graph:
        //     0       1
        //       \   /   \
        //         3      2
        //       /   \
        //      4     5
        assert!(list.order() == 6);
        // assert!(list.size() == 5); // TODO: uncomment when fixed
        assert!(list.has_edge(3, 5));
        assert!(list.connected());
    }

    #[test]
    fn undirected_graph_add_new_node_and_loop_edge() {
        // Graph:
        //     0       1
        //       \   /   \
        //         3      2
        //       /
        //      4
        let mut list = AdjacencyList(vec![vec![3], vec![2, 3], vec![1], vec![0, 1, 4], vec![3]]);
        list.add_node(5);
        list.add_undirected_edge(3, 5);
        list.add_undirected_edge(2, 2);

        // Current graph:
        //     0       1      -
        //       \   /   \  /   \
        //         3      2      |
        //       /   \      \    /
        //      4     5       -
        assert!(list.order() == 6);
        // assert!(list.size() == 6);// TODO: uncomment when fixed
        assert!(list.connected());
        assert!(list.has_edge(3, 5));
        assert!(list.has_edge(2, 2));
    }

    #[test]
    fn undirected_graph_remove_edge() {
        // Graph:
        //     0       1
        //       \   /   \
        //         3      2
        //       /   \
        //      4     5
        let mut list = AdjacencyList(vec![
            vec![3],
            vec![2, 3],
            vec![1],
            vec![0, 1, 4, 5],
            vec![3],
            vec![3],
        ]);
        list.remove_undirected_edge(1, 3);

        // Current graph:
        //     0       1
        //       \       \
        //         3      2
        //       /   \
        //      4     5
        assert!(list.order() == 6);
        // assert!(list.size() == 4); // TODO: uncomment when fixed
        assert!(!list.has_edge(3, 1));
        assert!(!list.connected());
    }

    #[test]
    fn undirected_graph_remove_edge_loop() {
        // Graph:
        //     0       1      -
        //       \   /   \  /   \
        //         3      2      |
        //       /   \      \    /
        //      4     5       -
        let mut list = AdjacencyList(vec![
            vec![3],
            vec![2, 3],
            vec![1, 2],
            vec![0, 1, 4, 5],
            vec![3],
            vec![3],
        ]);
        list.remove_undirected_edge(2, 2);

        // Current graph:
        //     0       1
        //       \    /  \
        //         3      2
        //       /   \
        //      4     5
        assert!(list.order() == 6);
        // assert!(list.size() == 5); // TODO: uncomment when fixed
        assert!(!list.has_edge(2, 2));
        assert!(list.has_edge(2, 1));
        assert!(list.connected());
    }

    #[test]
    fn undirected_graph_remove_node_3() {
        // Graph:
        //     0       1
        //       \   /   \
        //         3      2
        //       /   \
        //      4     5
        let mut list = AdjacencyList(vec![
            vec![3],
            vec![2, 3],
            vec![1],
            vec![0, 1, 4, 5],
            vec![3],
            vec![3],
        ]);
        list.remove_node(3);

        // Current graph:
        //     0       1
        //              \
        //               2
        //
        //       3    4
        assert!(list.order() == 5);
        // assert!(list.size() == 1); // TODO: uncomment when fixed
        assert!(!list.has_edge(0, 3));
        assert!(list.has_edge(2, 1));
        assert!(!list.connected());
    }
    #[test]
    fn undirected_graph_remove_node_4() {
        // Graph:
        //     0       1
        //       \   /   \
        //         3      2
        //       /   \
        //      4     5
        let mut list = AdjacencyList(vec![
            vec![3],
            vec![2, 3],
            vec![1],
            vec![0, 1, 4, 5],
            vec![3],
            vec![3],
        ]);
        list.remove_node(4);

        // Current graph:
        //     0       1
        //       \    /  \
        //         3      2
        //          \
        //           4
        assert!(list.order() == 5);
        // assert!(list.size() == 4); // TODO: uncomment when fixed
        assert!(list.has_edge(0, 3));
        assert!(list.connected());
    }

    #[test]
    fn node_degree_adjacency_list() {
        // Grafo: 0 ── 1 ── 2
        let graph = AdjacencyList(vec![vec![1], vec![0, 2], vec![1]]);

        assert_eq!(graph.node_degree(0), 1);
        assert_eq!(graph.node_degree(1), 2);
        assert_eq!(graph.node_degree(2), 1);
    }
}
