use std::marker::PhantomData;

use crate::graph::{
    DfsEvent, Directed, Direction, FromGraph, GraphError, Undirected, UndirectedGraph,
};
use crate::graph_io::UndirectedGraphIO;
use crate::graphs::{AdjacencyMatrix, IncidenceMatrix};
use crate::{Graph, GraphIO};

// #[derive(Debug,  Clone, Default)]
// pub struct AdjacencyList(pub Vec<Vec<usize>>);

#[derive(Debug, PartialEq, Clone)]
pub struct AdjacencyList<D: Direction> {
    data: Vec<Vec<usize>>,
    _marker: PhantomData<D>,
}

impl<D: Direction> Default for AdjacencyList<D> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            _marker: PhantomData,
        }
    }
}

impl<D: Direction> Graph<usize> for AdjacencyList<D> {
    fn new_empty() -> Self {
        AdjacencyList::new(&vec![]).unwrap()
    }

    fn order(&self) -> usize {
        self.data.len()
    }

    fn size(&self) -> usize {
        self.data.iter().map(|neighbors| neighbors.len()).sum()
    }

    fn nodes(&self) -> impl Iterator<Item = usize> {
        0..self.order()
    }

    fn underlying_graph(&self) -> Self {
        // directed
        let mut list = Self {
            data: vec![Vec::new(); self.data.len()],
            _marker: PhantomData,
        };

        for (idx_r, row) in self.data.iter().enumerate() {
            for &col in row.iter() {
                if !list.has_edge(idx_r, col) {
                    list.add_edge(idx_r, col);
                    list.add_edge(col, idx_r);
                }
            }
        }
        list
    }

    fn add_node(&mut self, _n: usize) {
        self.data.push(Vec::new());
    }

    fn remove_node(&mut self, n: usize) {
        if n < self.data.len() {
            self.data.remove(n);
            for neighbors in self.data.iter_mut() {
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
        if self.data.get(m).is_some()
            && let Some(n_edges) = self.data.get_mut(n)
            && !n_edges.contains(&(m))
        {
            n_edges.push(m);
        }
    }

    fn remove_edge(&mut self, n: usize, m: usize) {
        if let Some(edges) = self.data.get_mut(n) {
            edges.retain(|&x| x != m);
        }
    }

    type Neighbors<'a>
        = std::iter::Copied<std::slice::Iter<'a, usize>>
    where
        D: 'a;

    fn neighbors<'a>(&'a self, n: usize) -> Self::Neighbors<'a> {
        match self.data.get(n) {
            Some(edges) => edges.iter().copied(),
            None => [].iter().copied(),
        }
    }

    fn biparted(&self) -> bool {
        let n = self.order();
        if n == 0 {
            return true;
        }

        let mut side = vec![None; n];

        for start in 0..n {
            if side[start].is_some() {
                continue;
            }
            side[start] = Some(0);
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start);

            while let Some(u) = queue.pop_front() {
                let u_side = side[u].unwrap();
                for v in self.neighbors(u) {
                    if side[v].is_none() {
                        side[v] = Some(1 - u_side);
                        queue.push_back(v);
                    } else if side[v] == Some(u_side) {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn node_degrees(&self, n: usize) -> (usize, usize) {
        let out_deg = self.data().get(n).map_or(0, |neighbors| neighbors.len());
        let in_deg = self
            .data()
            .iter()
            .filter(|neighbors| neighbors.contains(&(n as i32)))
            .count();
        (in_deg, out_deg)
    }

    fn new(data: &Vec<Vec<i32>>) -> Result<Self, GraphError> {
        for (i, node) in data.iter().enumerate() {
            for &edge in node {
                if edge as usize >= data.len() || edge < 0 {
                    return Err(GraphError::InvalidLine(node.clone()));
                }

                if !(D::is_directed()) {
                    // TODO: cast data to usize
                    let correspondent = data[edge as usize].iter().find(|&&x| x == i as i32);
                    let has_correspondent = match correspondent {
                        None => false,
                        Some(_) => true,
                    };

                    if !has_correspondent {
                        return Err(GraphError::InvalidLine(node.clone()));
                    }
                }
            }
        }

        let data_usize: Vec<Vec<usize>> = data
            .iter()
            .map(|row| row.iter().map(|&x| x as usize).collect())
            .collect();

        Ok(Self {
            data: data_usize,
            _marker: PhantomData,
        })
    }

    fn data(&self) -> Vec<Vec<i32>> {
        self.data
            .iter()
            .map(|inner| inner.iter().map(|&x| x as i32).collect())
            .collect()
    }
}

impl<D: Direction> UndirectedGraph<usize> for AdjacencyList<D> {
    fn undirected_size(&self) -> usize {
        let mut self_loops = 0;
        let regular_edges: usize = self
            .data
            .iter()
            .enumerate()
            .map(|(i, _)| {
                self.neighbors(i)
                    .filter(|&n| {
                        let is_self_loop = n == i;
                        self_loops += is_self_loop as usize;
                        !is_self_loop
                    })
                    .count()
            })
            .sum();
        regular_edges / 2 + self_loops
    }

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

    fn undirected_node_degree(&self, node: usize) -> usize {
        self.data
            .get(node)
            .map(|neighbors| neighbors.len())
            .unwrap_or(0)
    }
}

impl<D: Direction> FromGraph<usize, IncidenceMatrix<D>> for AdjacencyList<D> {
    fn from_graph(g: &IncidenceMatrix<D>) -> Self {
        let mut list: Vec<Vec<i32>> = vec![vec![]; g.order()];

        for neighbors in g.data().iter() {
            let mut nodes = neighbors
                .into_iter()
                .enumerate()
                .take_while(|&(_, &x)| x != 0);

            let (edge_1, &weight_1) = nodes.next().unwrap();
            let (edge_2, &weight_2) = nodes.next().unwrap();

            if weight_1 < weight_2 {
                list[edge_1].push(edge_2 as i32);

                if !D::is_directed() {
                    list[edge_2].push(edge_1 as i32);
                }
            } else {
                list[edge_2].push(edge_1 as i32);

                if !D::is_directed() {
                    list[edge_1].push(edge_2 as i32);
                }
            }
        }

        AdjacencyList::new(&list).unwrap()
    }
}

impl<D: Direction> FromGraph<usize, AdjacencyMatrix<D>> for AdjacencyList<D> {
    fn from_graph(g: &AdjacencyMatrix<D>) -> Self {
        let mut adjacency_list = vec![Vec::new(); g.order()];

        for (i, row) in g.data().iter().enumerate() {
            adjacency_list[i].extend(
                row.iter()
                    .enumerate()
                    .filter_map(|(j, &val)| (val != 0).then_some(j)),
            );
        }
        let adjacency_list_i32: Vec<Vec<i32>> = adjacency_list
            .into_iter()
            .map(|inner| inner.into_iter().map(|x| x as i32).collect())
            .collect();
        AdjacencyList::new(&adjacency_list_i32).unwrap()
    }
}

impl<D: Direction> GraphIO<usize> for AdjacencyList<D> {}
impl<D: Direction> UndirectedGraphIO<usize> for AdjacencyList<D> {}

#[cfg(test)]
mod tests {

    use std::io::{Error, ErrorKind};

    use super::*;

    static PATH: &str = "examples/data/";

    #[test]
    fn new_digraph_1() {
        let result: Result<AdjacencyList<Directed>, Error> =
            GraphIO::import_from_file(PATH.to_owned() + "DIGRAFO1.txt");

        assert!(result.is_ok());

        match result {
            Ok(matrix) => {
                let g: AdjacencyList<Directed> = matrix;

                assert!(g.order() == 13);
                assert!(g.size() == 16);
            }
            Err(_) => {}
        }
    }

    #[test]
    fn new_digraph_2() {
        let result: Result<AdjacencyList<Directed>, Error> =
            GraphIO::import_from_file(PATH.to_owned() + "DIGRAFO2.txt");

        assert!(result.is_ok());

        match result {
            Ok(matrix) => {
                let g: AdjacencyList<Directed> = matrix;

                assert!(g.order() == 13);
                assert!(g.size() == 17);
            }
            Err(_) => {}
        }
    }

    #[test]
    fn new_digraph_error_1() {
        let result: Result<AdjacencyList<Undirected>, Error> =
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
        let res: Result<AdjacencyList<Directed>, Error> =
            UndirectedGraphIO::import_undirected_from_file(PATH.to_owned() + "GRAFO_2.txt");

        assert!(res.is_ok());

        match res {
            Ok(list) => {
                let g: AdjacencyList<Directed> = list;

                assert!(g.order() == 11);
                assert!(g.undirected_size() == 13);
            }
            Err(_) => {}
        }
    }

    #[test]
    fn new_undirected_graph_2() {
        let res: Result<AdjacencyList<Directed>, Error> =
            UndirectedGraphIO::import_undirected_from_file(PATH.to_owned() + "GRAFO_0.txt");

        assert!(res.is_err());
    }

    #[test]
    fn connected_undirected_graph() {
        // Graph: 2 ── 0 ── 1
        // should be connected.

        let list = vec![vec![1, 2], vec![0], vec![0]];
        let g: AdjacencyList<Undirected> = AdjacencyList::new(&list).unwrap();
        assert!(g.connected())
    }

    #[test]
    fn unconnected_undirected_graph() {
        // Graph: 2    0 ── 1
        // should be not connected.

        let list = vec![vec![1], vec![0], vec![]];
        let g: AdjacencyList<Undirected> = AdjacencyList::new(&list).unwrap();
        assert!(!g.connected());
    }

    #[test]
    fn underlying_graph_conversion() {
        // Graph:
        //     0      -> 1
        //       \    /   \
        //        -> 3     -> 2
        //       /
        //      4
        let original_list: AdjacencyList<Directed> =
            AdjacencyList::new(&vec![vec![3], vec![2], vec![], vec![1], vec![3]]).unwrap();
        let underlying_list = original_list.underlying_graph();

        // Current graph:
        //     0        - 1
        //       \    /    \
        //        - 3       - 2
        //       /
        //      4
        assert_eq!(original_list.order(), underlying_list.order());
        assert_eq!(original_list.size(), underlying_list.undirected_size());
        assert!(underlying_list.connected());
    }

    #[test]
    fn underlying_graph_conversion_and_node_delete_after() {
        // Graph:
        // 0 -> 1 -> 2 <- 3
        //      \    ^
        //       \   |
        //       ->  4
        let original_list: AdjacencyList<Directed> =
            AdjacencyList::new(&vec![vec![1], vec![2, 4], vec![], vec![2], vec![2]]).unwrap();
        let mut underlying_list = original_list.underlying_graph();

        // Current graph:
        // 0 -- 1 -- 2 -- 3
        //      \    |
        //       \   |
        //        -  4
        assert_eq!(original_list.order(), underlying_list.order());
        assert_eq!(original_list.size(), underlying_list.undirected_size());
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
        let mut list: AdjacencyList<Directed> =
            AdjacencyList::new(&vec![vec![3], vec![2], vec![], vec![1], vec![3]]).unwrap();
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
        let mut list: AdjacencyList<Directed> =
            AdjacencyList::new(&vec![vec![3], vec![2], vec![], vec![1], vec![3]]).unwrap();
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
        assert!(list.underlying_graph().connected());
    }

    #[test]
    fn graph_add_new_node_and_loop_edge() {
        // Graph:
        //     0      -> 1
        //       \    /   \
        //        -> 3     -> 2
        //       /
        //      4
        let mut list: AdjacencyList<Directed> =
            AdjacencyList::new(&vec![vec![3], vec![2], vec![], vec![1], vec![3]]).unwrap();
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
        assert!(list.underlying_graph().connected());
    }

    #[test]
    fn graph_remove_edge() {
        // Graph:
        //     0      -> 1
        //       \    /   \
        //        -> 3     -> 2
        //       /    \
        //      4      -> 5
        let mut list: AdjacencyList<Directed> =
            AdjacencyList::new(&vec![vec![3], vec![2], vec![], vec![1, 5], vec![3], vec![]])
                .unwrap();

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
        assert!(!list.underlying_graph().connected());
    }

    #[test]
    fn graph_remove_edge_loop() {
        // Graph:
        //     0      -> 1      ---
        //       \    /   \    /   \
        //        -> 3     -> 2    |
        //       /    \        ^   /
        //      4      -> 5    \--
        let mut list: AdjacencyList<Directed> = AdjacencyList::new(&vec![
            vec![3],
            vec![2],
            vec![2],
            vec![1, 5],
            vec![3],
            vec![],
        ])
        .unwrap();
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
        assert!(list.underlying_graph().connected());
    }

    #[test]
    fn graph_remove_node() {
        // Graph:
        //     0      -> 1
        //       \    /   \
        //        -> 3     -> 2
        //       /    \
        //      4      -> 5
        let mut list: AdjacencyList<Directed> =
            AdjacencyList::new(&vec![vec![3], vec![2], vec![], vec![1, 5], vec![3], vec![]])
                .unwrap();
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
        assert!(!list.underlying_graph().connected());
    }

    #[test]
    fn undirected_graph_add_new_node() {
        // Graph:
        //     0       1
        //       \   /   \
        //         3      2
        //       /
        //      4
        let mut list: AdjacencyList<Undirected> =
            AdjacencyList::new(&vec![vec![3], vec![2, 3], vec![1], vec![0, 1, 4], vec![3]])
                .unwrap();
        list.add_node(5);

        // Current graph:
        //     0       1
        //       \   /   \
        //         3      2
        //       /
        //      4     5
        assert!(list.order() == 6);
        assert!(list.undirected_size() == 4);
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
        let mut list: AdjacencyList<Undirected> =
            AdjacencyList::new(&vec![vec![3], vec![2, 3], vec![1], vec![0, 1, 4], vec![3]])
                .unwrap();
        list.add_node(5);
        list.add_undirected_edge(3, 5);

        // Current graph:
        //     0       1
        //       \   /   \
        //         3      2
        //       /   \
        //      4     5
        assert!(list.order() == 6);
        assert!(list.undirected_size() == 5);
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
        let mut list: AdjacencyList<Undirected> =
            AdjacencyList::new(&vec![vec![3], vec![2, 3], vec![1], vec![0, 1, 4], vec![3]])
                .unwrap();
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
        assert!(list.undirected_size() == 6);
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
        let mut list: AdjacencyList<Undirected> = AdjacencyList::new(&vec![
            vec![3],
            vec![2, 3],
            vec![1],
            vec![0, 1, 4, 5],
            vec![3],
            vec![3],
        ])
        .unwrap();
        list.remove_undirected_edge(1, 3);

        // Current graph:
        //     0       1
        //       \       \
        //         3      2
        //       /   \
        //      4     5
        assert!(list.order() == 6);
        assert!(list.undirected_size() == 4);
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
        let mut list: AdjacencyList<Undirected> = AdjacencyList::new(&vec![
            vec![3],
            vec![2, 3],
            vec![1, 2],
            vec![0, 1, 4, 5],
            vec![3],
            vec![3],
        ])
        .unwrap();
        list.remove_undirected_edge(2, 2);

        // Current graph:
        //     0       1
        //       \    /  \
        //         3      2
        //       /   \
        //      4     5
        assert!(list.order() == 6);
        assert!(list.undirected_size() == 5);
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
        let mut list: AdjacencyList<Undirected> = AdjacencyList::new(&vec![
            vec![3],
            vec![2, 3],
            vec![1],
            vec![0, 1, 4, 5],
            vec![3],
            vec![3],
        ])
        .unwrap();
        list.remove_node(3);

        // Current graph:
        //     0       1
        //              \
        //               2
        //
        //       3    4
        assert!(list.order() == 5);
        assert!(list.undirected_size() == 1);
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
        let mut list: AdjacencyList<Undirected> = AdjacencyList::new(&vec![
            vec![3],
            vec![2, 3],
            vec![1],
            vec![0, 1, 4, 5],
            vec![3],
            vec![3],
        ])
        .unwrap();
        list.remove_node(4);

        // Current graph:
        //     0       1
        //       \    /  \
        //         3      2
        //          \
        //           4
        assert!(list.order() == 5);
        assert!(list.undirected_size() == 4);
        assert!(list.has_edge(0, 3));
        assert!(list.connected());
    }

    #[test]
    fn node_degree_adjacency_list() {
        // Graph: 0 ── 1 ── 2
        let list: AdjacencyList<Undirected> =
            AdjacencyList::new(&vec![vec![1], vec![0, 2], vec![1]]).unwrap();

        assert_eq!(list.undirected_node_degree(0), 1);
        assert_eq!(list.undirected_node_degree(1), 2);
        assert_eq!(list.undirected_node_degree(2), 1);
    }

    #[test]
    fn adjacency_list_order() {
        // Graph: 0 ── 1
        //        │
        //        2
        let list: AdjacencyList<Undirected> =
            AdjacencyList::new(&vec![vec![1, 2], vec![0], vec![0]]).unwrap();
        assert_eq!(list.order(), 3);
    }

    #[test]
    fn test_size_adjacency_list() {
        // Graph: 0 ── 1
        //        │
        //        2
        let list: AdjacencyList<Undirected> =
            AdjacencyList::new(&vec![vec![1, 2], vec![0], vec![0]]).unwrap();

        assert_eq!(list.undirected_size(), 2);
    }

    #[test]
    fn test_node_degrees_list() {
        let mut list: AdjacencyList<Directed> = AdjacencyList::default();
        list.add_node(0);
        list.add_node(1);
        list.add_node(2);
        list.add_edge(0, 1);
        list.add_edge(1, 2);
        list.add_edge(2, 0);

        let degrees_0 = list.node_degrees(0);
        let degrees_1 = list.node_degrees(1);
        let degrees_2 = list.node_degrees(2);

        assert_eq!(degrees_0, (1, 1)); // in: 2->0, out: 0->1
        assert_eq!(degrees_1, (1, 1)); // in: 0->1, out: 1->2
        assert_eq!(degrees_2, (1, 1)); // in: 1->2, out: 2->0
    }
}
