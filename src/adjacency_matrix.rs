use crate::graph_io::UndirectedGraphIO;
use crate::graphs::{AdjacencyList, IncidenceMatrix};
use crate::{Graph, GraphIO, UndirectedGraph};

#[derive(Debug, Clone)]
pub struct AdjacencyMatrix(pub Vec<Vec<usize>>);

impl AdjacencyMatrix {
    pub fn from_adjacency_list(_list: &AdjacencyList) -> Self {
        let n = _list.0.len();
        let mut adjacency_matrix: Vec<Vec<usize>> = vec![vec![0; n]; n];

        for (i, neighbors) in _list.0.iter().enumerate() {
            for &j in neighbors {
                adjacency_matrix[i][j] = 1;
            }
        }
        AdjacencyMatrix(adjacency_matrix)
    }

    pub fn from_incidency_matrix(_matrix: &IncidenceMatrix) -> Self {
        todo!()
    }
}

impl Graph<usize> for AdjacencyMatrix {
    fn new_empty() -> Self {
        AdjacencyMatrix(vec![])
    }

    fn order(&self) -> usize {
        self.0.len()
    }

    fn size(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(i, _)| self.neighbors(i).count())
            .sum()
    }

    fn nodes(&self) -> impl Iterator<Item = usize> {
        0..self.order()
    }

    fn underlying_graph(&self) -> Self {
        let mut matrix: AdjacencyMatrix =
            AdjacencyMatrix(vec![vec![0; self.0.len()]; self.0.len()]);

        for (idx_r, row) in self.0.iter().enumerate() {
            for (idx_c, col) in row.iter().enumerate() {
                if *col == 1 && !matrix.has_edge(idx_c, idx_r) {
                    matrix.add_undirected_edge(idx_r, idx_c);
                }
            }
        }

        matrix
    }

    fn add_node(&mut self, _n: usize) {
        self.0.push(Vec::new());
        let new_order = self.order();

        for r in &mut self.0 {
            while r.len() < new_order {
                r.push(0);
            }
        }
    }

    /// Adds a new edge between two nodes `n` and `m`
    fn add_edge(&mut self, n: usize, m: usize) {
        if let Some(edges) = self.0.get_mut(n)
            && let Some(edge) = edges.get_mut(m)
        {
            if *edge == 1 {
                return;
            }
            *edge = 1;
        }
    }

    // Removes a node and its edges by its index
    fn remove_node(&mut self, n: usize) {
        if n < self.0.len() {
            self.0.remove(n);
            for row in self.0.iter_mut() {
                for idx in n + 1..row.len() {
                    row[idx - 1] = row[idx];
                }
                row.pop();
            }
        }
    }

    fn remove_edge(&mut self, n: usize, m: usize) {
        if let Some(edges) = self.0.get_mut(n)
            && let Some(edge) = edges.get_mut(m)
        {
            *edge = 0;
        }
    }

    type Neighbors<'a> = std::iter::FilterMap<
        std::iter::Enumerate<std::slice::Iter<'a, usize>>,
        fn((usize, &'a usize)) -> Option<usize>,
    >;

    fn neighbors<'a>(&'a self, n: usize) -> Self::Neighbors<'a> {
        fn filter_fn((i, &weight): (usize, &usize)) -> Option<usize> {
            if weight != 0 { Some(i) } else { None }
        }
        match self.0.get(n) {
            Some(row) => row.iter().enumerate().filter_map(filter_fn),
            None => [].iter().enumerate().filter_map(filter_fn),
        }
    }

    fn biparted(&self) -> bool {
        todo!()
    }

    fn node_degrees(&self, n: usize) -> (usize, usize) {
        let out_deg = self.0[n].iter().filter(|&&v| v != 0).count();
        let in_deg = self.0.iter().filter(|row| row[n] != 0).count();
        (in_deg, out_deg)
    }
}

impl UndirectedGraph<usize> for AdjacencyMatrix {
    fn undirected_size(&self) -> usize {
        let mut size = 0;
        for i in 0..self.order() {
            for j in 0..=i {
                if self.0[i][j] > 0 {
                    size += 1;
                }
            }
        }
        size
    }

    fn connected(&self) -> bool {
        let n = self.order();
        if n == 0 {
            return true;
        }

        let mut visited = vec![false; n];
        let mut stack = vec![0];
        visited[0] = true;

        while let Some(u) = stack.pop() {
            for (v, &is_edge) in self.0[u].iter().enumerate() {
                if is_edge > 0 && !visited[v] {
                    visited[v] = true;
                    stack.push(v);
                }
            }
        }

        visited.into_iter().all(|v| v)
    }

    fn undirected_node_degree(&self, node: usize) -> usize {
        if let Some(row) = self.0.get(node) {
            row.iter().filter(|&&val| val != 0).count()
        } else {
            0
        }
    }
}

impl GraphIO<usize> for AdjacencyMatrix {}

impl UndirectedGraphIO<usize> for AdjacencyMatrix {}

#[cfg(test)]
mod tests {
    use std::{
        io::{Error, ErrorKind},
        vec,
    };

    use super::*;

    static PATH: &str = "examples/data/";

    #[test]
    fn new_digraph_1() {
        let result: Result<AdjacencyMatrix, Error> =
            GraphIO::import_from_file(PATH.to_owned() + "DIGRAFO1.txt");

        assert!(result.is_ok());

        match result {
            Ok(matrix) => {
                assert!(matrix.order() == 13);
                assert!(matrix.size() == 16);
            }
            Err(_) => {}
        }
    }

    #[test]
    fn new_digraph_2() {
        let result: Result<AdjacencyMatrix, Error> =
            GraphIO::import_from_file(PATH.to_owned() + "DIGRAFO2.txt");

        assert!(result.is_ok());

        match result {
            Ok(matrix) => {
                assert!(matrix.order() == 13);
                assert!(matrix.size() == 17);
            }
            Err(_) => {}
        }
    }

    #[test]
    fn new_digraph_error_1() {
        let result: Result<AdjacencyMatrix, Error> =
            GraphIO::import_from_file(PATH.to_owned() + "GRAFO_0.txt");

        assert!(result.is_err());

        match result {
            Ok(_) => {}
            Err(err) => {
                assert!(err.kind() == ErrorKind::InvalidData);
                assert!(err.to_string().contains("Invalid data was found"));
            }
        }
    }

    #[test]
    fn new_undirected_graph_1() {
        let res: Result<AdjacencyMatrix, Error> =
            UndirectedGraphIO::import_undirected_from_file(PATH.to_owned() + "GRAFO_2.txt");

        assert!(res.is_ok());

        match res {
            Ok(list) => {
                assert!(list.order() == 11);
                assert!(list.undirected_size() == 13);
            }
            Err(_) => {}
        }
    }

    #[test]
    fn new_undirected_graph_2() {
        let res: Result<AdjacencyMatrix, Error> =
            UndirectedGraphIO::import_undirected_from_file(PATH.to_owned() + "GRAFO_0.txt");

        assert!(res.is_err());
    }

    #[test]
    fn undirected_graph_matrix_size() {
        let undirected_m = AdjacencyMatrix(vec![
            vec![1, 1, 0, 1, 0, 0],
            vec![1, 0, 1, 1, 0, 0],
            vec![0, 1, 0, 1, 0, 0],
            vec![1, 1, 1, 0, 1, 1],
            vec![0, 0, 0, 1, 0, 1],
            vec![0, 0, 0, 1, 1, 0],
        ]);
        assert_eq!(undirected_m.undirected_size(), 9);
    }

    #[test]
    fn graph_matrix_size() {
        let directed_m = AdjacencyMatrix(vec![
            vec![1, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0],
            vec![1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 1],
            vec![0, 0, 0, 1, 0, 0],
        ]);
        assert_eq!(directed_m.size(), 9);
    }

    #[test]
    fn adjacency_list_to_matrix() {
        // Grafo: 0 ── 1
        //        │
        //        2
        let list = AdjacencyList(vec![vec![1, 2], vec![0], vec![0]]);
        let matrix = AdjacencyMatrix::from_adjacency_list(&list);

        assert_eq!(matrix.0, vec![vec![0, 1, 1], vec![1, 0, 0], vec![1, 0, 0]]);
    }

    #[test]
    fn matrix_to_list() {
        // Mesmo grafo de cima, mas em matriz
        let matrix = AdjacencyMatrix(vec![vec![0, 1, 1], vec![1, 0, 0], vec![1, 0, 0]]);

        let list = AdjacencyList::from_adjacency_matrix(&matrix);

        assert_eq!(list.0, vec![vec![1, 2], vec![0], vec![0]]);
    }

    #[test]
    fn round_trip_conversion() {
        // Grafo: 0 ── 1 ── 2
        let original_list = AdjacencyList(vec![vec![1], vec![0, 2], vec![1]]);
        let matrix = AdjacencyMatrix::from_adjacency_list(&original_list);
        let converted_list = AdjacencyList::from_adjacency_matrix(&matrix);

        assert_eq!(original_list.0, converted_list.0);
    }

    #[test]
    fn underlying_graph_conversion() {
        // Graph:
        // 0 -> 1 -> 2 <- 3
        //      \    ^
        //       \   |
        //       ->  4
        let original_graph = AdjacencyMatrix(vec![
            vec![0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0],
        ]);

        // Current graph:
        // 0 -- 1 -- 2 -- 3
        //      \    |
        //       \   |
        //        -  4
        let underlying_graph = original_graph.underlying_graph();
        assert_eq!(original_graph.order(), underlying_graph.order());
    }

    #[test]
    fn graph_add_new_node() {
        // Graph: 0 -> 2 <- 1
        let mut m = AdjacencyMatrix(vec![vec![0, 0, 1], vec![0, 0, 1], vec![0, 0, 0]]);
        m.add_node(3);
        // Graph: 0 -> 2 <- 1  3
        assert!(m.order() == 4);
        // assert!(!m.underlying_graph().connected());
    }

    #[test]
    fn graph_add_new_node_and_edge() {
        // Graph: 0 -> 2 <- 1
        let mut m = AdjacencyMatrix(vec![vec![0, 0, 1], vec![0, 0, 1], vec![0, 0, 0]]);
        m.add_node(3);
        m.add_edge(1, 3);
        // Graph: 0 -> 2 <- 1 -> 3
        assert!(m.has_edge(1, 3));
        assert!(!m.has_edge(3, 1));
        // assert!(m.underlying_graph().connected());
    }

    #[test]
    fn undirected_graph_add_new_node_and_edge() {
        // Graph: 0 - 2 - 1
        let mut m = AdjacencyMatrix(vec![vec![0, 0, 1], vec![0, 0, 1], vec![1, 1, 0]]);
        m.add_node(3);
        m.add_undirected_edge(1, 3);
        // Graph: 0 - 2 - 1 - 3
        assert!(m.has_edge(1, 3));
        assert!(m.has_edge(3, 1));
        assert_eq!(m.undirected_size(), 3);
        // assert!(!m.underlying_graph().connected());
    }

    #[test]
    fn graph_remove_edge() {
        // Graph:
        // 0 -> 1 -> 2 <- 3
        //      \    ^
        //       \   |
        //       ->  4
        let mut m = AdjacencyMatrix(vec![
            vec![0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0],
        ]);

        m.remove_edge(1, 4);
        assert!(!m.has_edge(1, 4));
        assert!(!m.has_edge(4, 1));
        assert!(m.size() == 4);
        // assert!(m.underlying_graph().connected());
    }

    #[test]
    fn graph_duplicate_remove_edge() {
        // Graph:
        // 0 -> 1 -> 2 <- 3
        //      \    ^
        //       \   |
        //       ->  4
        let mut m = AdjacencyMatrix(vec![
            vec![0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0],
        ]);

        m.remove_edge(1, 4);
        m.remove_edge(1, 4);
        assert!(!m.has_edge(1, 4));
        assert!(!m.has_edge(4, 1));
        assert!(m.size() == 4);
        // assert!(m.underlying_graph().connected());
    }

    #[test]
    fn graph_remove_node() {
        // Graph:
        // 0 -> 1 -> 2 <- 3
        //      \    ^
        //       \   |
        //       ->  4
        let mut m = AdjacencyMatrix(vec![
            vec![0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0],
        ]);

        m.remove_node(2);
        assert!(m.order() == 4);
        assert!(m.size() == 2);
        assert!(!m.has_edge(3, 2));
        assert!(!m.has_edge(1, 2));
        assert!(!m.has_edge(4, 2));
    }

    #[test]
    fn graph_remove_node_and_add_new() {
        // Graph:
        // 0 -> 1 -> 2 <- 3
        //      \    ^
        //       \   |
        //       ->  4
        let mut m = AdjacencyMatrix(vec![
            vec![0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0],
        ]);

        m.remove_node(2);

        m.add_node(0);

        m.add_edge(2, 4);
        m.add_edge(2, 3);

        // Graph:
        // 0 -> 1     2
        //   \        | \
        //   \       |  \
        //   -> 3 <-     -> 4
        assert!(m.order() == 5);
        assert!(m.size() == 4);
        assert!(m.has_edge(1, 3));
        assert!(m.has_edge(2, 3));
        assert!(m.has_edge(2, 4));
        assert!(!m.has_edge(4, 2));
    }

    #[test]
    fn undirected_graph_remove_edge() {
        // Graph:
        // 0 -- 1 -- 2 -- 3
        //      \    |
        //       \   |
        //       --  4
        let mut m = AdjacencyMatrix(vec![
            vec![0, 1, 0, 0, 0],
            vec![1, 0, 1, 0, 1],
            vec![0, 1, 0, 1, 1],
            vec![0, 0, 1, 0, 0],
            vec![0, 1, 1, 0, 0],
        ]);

        m.remove_undirected_edge(2, 4);
        assert!(!m.has_edge(2, 4));
        assert!(!m.has_edge(4, 2));
        assert!(m.undirected_size() == 4);
    }

    #[test]
    fn undirected_graph_remove_node() {
        // Graph:
        // 0 -- 1 -- 2 -- 3
        //      \    |
        //       \   |
        //       --  4
        let mut m = AdjacencyMatrix(vec![
            vec![0, 1, 0, 0, 0],
            vec![1, 0, 1, 0, 1],
            vec![0, 1, 0, 1, 1],
            vec![0, 0, 1, 0, 0],
            vec![0, 1, 1, 0, 0],
        ]);

        m.remove_node(4);
        assert_eq!(m.undirected_size(), 3);
        assert!(!m.has_edge(2, 4));
        assert!(!m.has_edge(1, 4));
    }

    #[test]
    fn node_degree_adjacency_matrix() {
        // Grafo: 0 ─ 1
        //        │ /
        //        2
        let matrix = AdjacencyMatrix(vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]]);

        assert_eq!(matrix.undirected_node_degree(0), 2);
        assert_eq!(matrix.undirected_node_degree(1), 2);
        assert_eq!(matrix.undirected_node_degree(2), 2);
    }

    #[test]
    fn adjacency_matrix_order() {
        // Graph: 0 ── 1
        //        │
        //        2
        let matrix = AdjacencyMatrix(vec![vec![0, 1, 1], vec![1, 0, 0], vec![1, 0, 0]]);
        assert_eq!(matrix.order(), 3);
    }

    #[test]
    fn test_connected_graph() {
        // Graph: 0 ─ 1
        //        │ /
        //        2
        let matrix = AdjacencyMatrix(vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]]);
        assert!(matrix.connected());
    }

    #[test]
    fn test_disconnected_graph() {
        // Graph: 0 ─ 1     2
        let matrix = AdjacencyMatrix(vec![vec![0, 1, 0], vec![1, 0, 0], vec![0, 0, 0]]);
        assert!(!matrix.connected());
    }

    #[test]
    fn test_node_degrees_matrix() {
        let mut matrix = AdjacencyMatrix(vec![vec![0, 0, 0]; 3]);
        matrix.0[0][1] = 1;
        matrix.0[1][2] = 1;
        matrix.0[2][0] = 1;

        let degrees_0 = matrix.node_degrees(0);
        let degrees_1 = matrix.node_degrees(1);
        let degrees_2 = matrix.node_degrees(2);

        assert_eq!(degrees_0, (1, 1)); // in: 2->0, out: 0->1
        assert_eq!(degrees_1, (1, 1)); // in: 0->1, out: 1->2
        assert_eq!(degrees_2, (1, 1)); // in: 1->2, out: 2->0
    }
}
