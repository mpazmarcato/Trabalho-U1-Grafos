use crate::graphs::{AdjacencyList, AdjacencyMatrix};
use crate::{Graph, UndirectedGraph};

/// Represents a graph using an incidence matrix.
/// Each row corresponds to an edge, and each column corresponds to a node.
/// Cell values indicate the relationship between the edge and the node:
/// `-1` for the source node, `1` for the target node, and `0` otherwise.
/// # Note
/// Many functions in this struct are currently `to-do`.
/// The focus is on adjacency list and adjacency matrix for now.
#[derive(Debug, Clone)]
pub struct IncidenceMatrix(pub Vec<Vec<i32>>);

impl IncidenceMatrix {
    /// Constructs an incidence matrix from an adjacency matrix.
    ///
    /// # Arguments
    /// * `matrix` - The adjacency matrix representing the graph.
    pub fn from_adjacency_matrix(matrix: &AdjacencyMatrix) -> Self {
        let n = matrix.0.len();

        let mut edges: Vec<(usize, usize)> = Vec::new();
        for i in 0..n {
            for j in 0..n {
                if matrix.0[i][j] == 1 {
                    edges.push((i, j));
                }
            }
        }

        let m = edges.len();
        let mut inc: Vec<Vec<i32>> = vec![vec![0; n]; m];

        for (i, &(l, c)) in edges.iter().enumerate() {
            inc[i][l] = -1;
            inc[i][c] = 1;
        }

        IncidenceMatrix(inc)
    }

    /// Constructs an incidence matrix from an adjacency list representing a directed graph.
    ///
    /// # Arguments
    /// * `adj_list` - The adjacency list of a directed graph.
    pub fn from_directed_adjacency_list(adj_list: &AdjacencyList) -> Self {
        let mut incidence_matrix: Vec<Vec<i32>> = Vec::new();

        for (c_out, v) in adj_list.0.iter().enumerate() {
            for c_in in v.iter() {
                let mut edge: Vec<i32> = vec![0; adj_list.order()];

                edge[c_out] = -1;
                edge[*c_in] = 1;

                incidence_matrix.push(edge);
            }
        }

        IncidenceMatrix(incidence_matrix)
    }

    /// Constructs an incidence matrix from an adjacency list representing an undirected graph.
    ///
    /// # Arguments
    /// * `adj_list` - The adjacency list of an undirected graph.
    pub fn from_undirected_adjacency_list(adj_list: &AdjacencyList) -> Self {
        let mut incidence_matrix: Vec<Vec<i32>> = Vec::new();

        for (c_out, v) in adj_list.0.iter().enumerate() {
            for c_in in v.iter() {
                let mut edge: Vec<i32> = vec![0; adj_list.order()];

                edge[c_out] = 1;
                edge[*c_in] = 1;

                incidence_matrix.push(edge);
            }
        }

        IncidenceMatrix(incidence_matrix)
    }
}

impl Graph<usize> for IncidenceMatrix {
    fn new_empty() -> Self {
        IncidenceMatrix(Vec::new())
    }

    fn order(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    fn size(&self) -> usize {
        self.0.len()
    }

    fn node_degrees(&self, _n: usize) -> (usize, usize) {
        todo!()
    }

    fn nodes(&self) -> impl Iterator<Item = usize> {
        0..self.order()
    }

    fn add_node(&mut self, _n: usize) {
        todo!()
    }

    fn remove_node(&mut self, _n: usize) {
        todo!()
    }

    fn add_edge(&mut self, _n: usize, _m: usize) {
        todo!()
    }

    fn remove_edge(&mut self, _n: usize, _m: usize) {
        todo!()
    }

    type Neighbors<'a> = std::iter::FilterMap<
        std::iter::Enumerate<std::slice::Iter<'a, usize>>,
        fn((usize, &'a usize)) -> Option<usize>,
    >;

    fn neighbors<'a>(&'a self, _n: usize) -> Self::Neighbors<'a> {
        todo!()
    }

    fn bipartite(&self) -> bool {
        let n = self.order();
        if n == 0 {
            return true;
        }

        let mut adj = vec![Vec::new(); n];

        for edge in &self.0 {
            let endpoints: Vec<usize> = edge
                .iter()
                .enumerate()
                .filter_map(|(v, &x)| if x != 0 { Some(v) } else { None })
                .collect();

            match endpoints.as_slice() {
                [u, v] => {
                    adj[*u].push(*v);
                    adj[*v].push(*u);
                }
                [u] => {
                    adj[*u].push(*u);
                }
                _ => {}
            }
        }

        let mut partition = vec![None; n];
        let mut queue = std::collections::VecDeque::new();

        for start in 0..n {
            if partition[start].is_some() {
                continue;
            }

            partition[start] = Some(0);
            queue.push_back(start);

            while let Some(u) = queue.pop_front() {
                let u_side = partition[u].unwrap();

                for &v in &adj[u] {
                    if partition[v].is_none() {
                        partition[v] = Some(1 - u_side);
                        queue.push_back(v);
                    } else if partition[v] == Some(u_side) {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn underlying_graph(&self) -> Self {
        todo!()
    }
}

impl UndirectedGraph<usize> for IncidenceMatrix {
    fn undirected_size(&self) -> usize {
        self.0.len()
    }

    fn connected(&self) -> bool {
        todo!()
    }

    fn undirected_node_degree(&self, vertex: usize) -> usize {
        if self.0.is_empty() || vertex >= self.0[0].len() {
            return 0;
        }

        self.0.iter().filter(|row| row[vertex] != 0).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undirected_node_degree() {
        // Graph: 0 ── 1 ── 2
        //
        // Edges:
        // e1: 0–1 → [1, 1, 0]
        // e2: 1–2 → [0, 1, 1]
        //
        // IncidenceMatrix:
        // [
        //   [1, 1, 0],
        //   [0, 1, 1]
        // ]
        let incidence = IncidenceMatrix(vec![vec![1, 1, 0], vec![0, 1, 1]]);

        assert_eq!(incidence.undirected_node_degree(0), 1); // connected (0–1)
        assert_eq!(incidence.undirected_node_degree(1), 2); // connected (0–1) e (1–2)
        assert_eq!(incidence.undirected_node_degree(2), 1); // connected to edge (1–2)
    }

    #[test]
    fn test_size_incidence_matrix_direct() {
        // Graph: 0 ── 1 ── 2
        let incidence = IncidenceMatrix(vec![
            vec![1, 1, 0], // 1 to 0 and 1
            vec![0, 1, 1], // 2 to 1 and 2
        ]);

        assert_eq!(incidence.size(), 2);
        assert_eq!(incidence.undirected_node_degree(0), 1);
        assert_eq!(incidence.undirected_node_degree(1), 2);
        assert_eq!(incidence.undirected_node_degree(2), 1);
    }

    #[test]
    fn from_adjacencylist_digraph() {
        //    (0)      ->(1)
        //       \a₁  /a₃  \a₂
        //        ->(3)     ->(2)
        //       /a₄
        //     (4)
        let adj_list = AdjacencyList(vec![vec![3], vec![2], vec![], vec![1], vec![3]]);
        let answer: Vec<Vec<i32>> = vec![
            vec![-1, 0, 0, 1, 0],
            vec![0, -1, 1, 0, 0],
            vec![0, 1, 0, -1, 0],
            vec![0, 0, 0, 1, -1],
        ];

        let incidence_matrix = IncidenceMatrix::from_directed_adjacency_list(&adj_list);

        assert_eq!(incidence_matrix.0, answer);
    }

    #[test]
    fn from_adjacencylist_graph() {
        //    (0)      --(1)
        //       \a₁  /a₃  \a₂
        //        --(3)     --(2)
        //       /a₄
        //     (4)
        let adjlist = AdjacencyList(vec![vec![3], vec![2], vec![], vec![1], vec![3]]);
        let answer: Vec<Vec<i32>> = vec![
            vec![1, 0, 0, 1, 0],
            vec![0, 1, 1, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 0, 0, 1, 1],
        ];

        let inc = IncidenceMatrix::from_undirected_adjacency_list(&adjlist);

        assert_eq!(inc.0, answer);
    }
}
