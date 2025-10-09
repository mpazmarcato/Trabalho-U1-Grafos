use std::clone;
use std::fs::File;
use std::io::{self, Write};
use std::marker::PhantomData;

use crate::graph::{Directed, Direction, FromGraph, GraphError, Undirected};
use crate::graphs::{AdjacencyList, IncidenceMatrix};
use crate::{Graph, UndirectedGraph};

#[derive(Debug)]
pub struct Node {
    value: usize,
    ancestor: Option<usize>,
}

// #[derive(Debug, Clone)]
// pub struct AdjacencyMatrix(pub Vec<Vec<usize>>);
#[derive(Debug, PartialEq, Clone)]
pub struct AdjacencyMatrix<D: Direction> {
    data: Vec<Vec<i32>>,
    _marker: PhantomData<D>,
}

impl<D: Direction> AdjacencyMatrix<D> {
    // TODO: Investigate why this is dead and when we should use it.
    #[allow(dead_code)]
    fn write_graph_to_dot(graph: &Vec<Node>, path: String) -> io::Result<()> {
        let mut file: File = File::create(path)?;

        writeln!(file, "digraph G {{")?;
        writeln!(file, "  rankdir=LR;")?;
        writeln!(file, "  node [shape=circle];")?;

        for node in graph {
            writeln!(file, "  {}", node.value)?;
        }

        for node in graph {
            if let Some(ancestor_idx) = node.ancestor {
                writeln!(file, "  {} -> {};", graph[ancestor_idx].value, node.value)?;
            }
        }

        writeln!(file, " }}")?;
        Ok(())
    }
}

impl<D: Direction> Graph<usize> for AdjacencyMatrix<D> {
    fn order(&self) -> usize {
        self.data.len()
    }

    fn size(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .map(|(i, _)| self.neighbors(i).count())
            .sum()
    }

    fn underlying_graph(&self) -> Self {
        let g = vec![vec![0; self.data.len()]; self.data.len()];
        let mut matrix = AdjacencyMatrix::<D>::new(&g).unwrap();

        for (idx_r, row) in self.data.iter().enumerate() {
            for (idx_c, col) in row.iter().enumerate() {
                if *col == 1 && !matrix.has_edge(idx_c, idx_r) {
                    matrix.add_edge(idx_r, idx_c);
                    matrix.add_edge(idx_r, idx_c);
                }
            }
        }

        matrix
    }

    fn add_node(&mut self, _n: usize) {
        self.data.push(Vec::new());
        let new_order = self.order();

        for r in &mut self.data {
            while r.len() < new_order {
                r.push(0);
            }
        }
    }

    /// Adds a new edge between two nodes `n` and `m`
    fn add_edge(&mut self, n: usize, m: usize) {
        if let Some(edges) = self.data.get_mut(n)
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
        if n < self.data.len() {
            self.data.remove(n);
            for row in self.data.iter_mut() {
                for idx in n + 1..row.len() {
                    row[idx - 1] = row[idx];
                }
                row.pop();
            }
        }
    }

    fn remove_edge(&mut self, n: usize, m: usize) {
        if let Some(edges) = self.data.get_mut(n)
            && let Some(edge) = edges.get_mut(m)
        {
            *edge = 0;
        }
    }

    type Neighbors<'a>
        = std::iter::FilterMap<
        std::iter::Enumerate<std::slice::Iter<'a, i32>>,
        fn((usize, &'a i32)) -> Option<usize>,
    >
    where
        D: 'a;

    fn neighbors<'a>(&'a self, n: usize) -> Self::Neighbors<'a> {
        fn filter_fn((i, &weight): (usize, &i32)) -> Option<usize> {
            if weight != 0 { Some(i) } else { None }
        }

        match self.data.get(n) {
            Some(row) => row.iter().enumerate().filter_map(filter_fn),
            None => (&[] as &[i32]).iter().enumerate().filter_map(filter_fn),
        }
    }

    fn biparted(&self) -> bool {
        todo!()
    }

    fn new(data: &Vec<Vec<i32>>) -> Result<Self, GraphError> {
        if data.is_empty() {
            return Ok(Self {
                data: data.clone(),
                _marker: PhantomData,
            });
        }

        let order = data[0].len();

        // vectors with diferent sizes means that some nodes are unreachable for certain edges
        for (i, node) in data.iter().enumerate() {
            if order != node.len() {
                return Err(GraphError::Dimensions);
            }
            if !D::is_directed() {
                for (j, edge) in node.iter().enumerate() {
                    if data[i][j] != data[j][i] {
                        return Err(GraphError::InvalidLine(node.clone()));
                    }
                }
            }
        }

        Ok(Self {
            data: data.clone(),
            _marker: PhantomData,
        })
    }

    fn data(&self) -> Vec<Vec<i32>> {
        (&self.data).clone()
    }
}

impl UndirectedGraph<usize> for AdjacencyMatrix<Undirected> {
    fn undirected_size(&self) -> usize {
        let mut size = 0;
        for i in 0..self.order() {
            for j in 0..=i {
                if self.data[i][j] > 0 {
                    size += 1;
                }
            }
        }
        size
    }

    fn add_undirected_edge(&mut self, n: usize, m: usize) {
        self.add_edge(n, m);
        self.add_edge(m, n);
    }

    fn connected(&self) -> bool {
        todo!()
    }

    fn undirected_node_degree(&self, node: usize) -> usize {
        if let Some(row) = self.data.get(node) {
            row.iter().filter(|&&val| val != 0).count()
        } else {
            0
        }
    }

    fn undirected_order(&self) -> usize {
        self.data.len()
    }
}

impl<D: Direction> FromGraph<usize, IncidenceMatrix<D>> for AdjacencyMatrix<D> {
    fn from_graph(g: &IncidenceMatrix<D>) -> Self {
        let mut adjacency_matrix: Vec<Vec<i32>> = vec![vec![0; g.order()]; g.order()];

        for (i, neighbors) in g.data().iter().enumerate() {
            let mut nodes = neighbors
                .into_iter()
                .enumerate()
                .take_while(|&(edge, &x)| x != 0);

            let (edge_1, &weight_1) = nodes.next().unwrap();
            let (edge_2, &weight_2) = nodes.next().unwrap();

            if weight_1 < weight_2 {
                adjacency_matrix[edge_1][edge_2] = weight_2;

                if !D::is_directed() {
                    adjacency_matrix[edge_2][edge_1] = weight_2;
                }
            } else {
                adjacency_matrix[edge_2][edge_1] = weight_1;

                if !D::is_directed() {
                    adjacency_matrix[edge_1][edge_2] = weight_1;
                }
            }
        }
        AdjacencyMatrix::new(&adjacency_matrix).unwrap()
    }
}

impl<D: Direction> FromGraph<usize, AdjacencyList<D>> for AdjacencyMatrix<D> {
    fn from_graph(g: &AdjacencyList<D>) -> Self {
        let n = g.order();
        let mut matrix = vec![vec![0; n]; n];

        for (i, neighbors) in g.data().iter().enumerate() {
            for &j in neighbors {
                matrix[i as usize][j as usize] = 1;
                if !D::is_directed() {
                    matrix[j as usize][i as usize] = 1;
                }
            }
        }

        AdjacencyMatrix::new(&matrix).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn undirected_graph_matrix_size() {
        let undirected_m = vec![
            vec![1, 1, 0, 1, 0, 0],
            vec![1, 0, 1, 1, 0, 0],
            vec![0, 1, 0, 1, 0, 0],
            vec![1, 1, 1, 0, 1, 1],
            vec![0, 0, 0, 1, 0, 1],
            vec![0, 0, 0, 1, 1, 0],
        ];
        let g: AdjacencyMatrix<Undirected> = AdjacencyMatrix::new(&undirected_m).unwrap();

        assert_eq!(g.undirected_size(), 9);
    }

    #[test]
    fn graph_matrix_size() {
        let directed_m = vec![
            vec![1, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0],
            vec![1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 1],
            vec![0, 0, 0, 1, 0, 0],
        ];
        let g: AdjacencyMatrix<Directed> = AdjacencyMatrix::new(&directed_m).unwrap();
        assert_eq!(g.size(), 9);
    }

    #[test]
    fn adjacency_list_to_matrix() {
        // Grafo: 0 ── 1
        //        │
        //        2
        let list = vec![vec![1, 2], vec![0], vec![0]];
        let g: AdjacencyList<Undirected> = AdjacencyList::new(&list).unwrap();
        let matrix = AdjacencyMatrix::from_graph(&g);

        assert_eq!(
            *matrix.data(),
            vec![vec![0, 1, 1], vec![1, 0, 0], vec![1, 0, 0]]
        );
    }

    #[test]
    fn matrix_to_list() {
        // Mesmo grafo de cima, mas em matriz
        let matrix = vec![vec![0, 1, 1], vec![1, 0, 0], vec![1, 0, 0]];
        let g: AdjacencyMatrix<Undirected> = AdjacencyMatrix::new(&matrix).unwrap();

        let list = AdjacencyList::from_graph(&g);

        assert_eq!(*list.data(), vec![vec![1, 2], vec![0], vec![0]]);
    }

    #[test]
    fn round_trip_conversion() {
        // Grafo: 0 ── 1 ── 2
        let original_list = vec![vec![1], vec![0, 2], vec![1]];
        let g: AdjacencyList<Undirected> = AdjacencyList::new(&original_list).unwrap();
        let matrix = AdjacencyMatrix::from_graph(&g);
        let converted_list = AdjacencyList::from_graph(&matrix);

        assert_eq!(*converted_list.data(), original_list);
    }

    #[test]
    fn underlying_graph_conversion() {
        // Graph:
        // 0 -> 1 -> 2 <- 3
        //      \    ^
        //       \   |
        //       ->  4
        let directed_m = vec![
            vec![0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0],
        ];
        let original_graph: AdjacencyMatrix<Directed> = AdjacencyMatrix::new(&directed_m).unwrap();

        let underlying_graph = original_graph.underlying_graph();
        assert_eq!(original_graph.order(), underlying_graph.order());
    }

    #[test]
    fn graph_add_new_node() {
        // Graph: 0 -> 2 <- 1
        let m = vec![vec![0, 0, 1], vec![0, 0, 1], vec![0, 0, 0]];
        let mut g: AdjacencyMatrix<Directed> = AdjacencyMatrix::new(&m).unwrap();
        g.add_node(3);
        assert_eq!(g.order(), 4);
    }

    #[test]
    fn graph_add_new_node_and_edge() {
        // Graph: 0 -> 2 <- 1
        let m = vec![vec![0, 0, 1], vec![0, 0, 1], vec![0, 0, 0]];
        let mut g: AdjacencyMatrix<Directed> = AdjacencyMatrix::new(&m).unwrap();
        g.add_node(3);
        g.add_edge(1, 3);
        assert!(g.has_edge(1, 3));
        assert!(!g.has_edge(3, 1));
    }

    #[test]
    fn undirected_graph_add_new_node_and_edge() {
        // Graph: 0 - 2 - 1
        let m = vec![vec![0, 0, 1], vec![0, 0, 1], vec![1, 1, 0]];
        let mut g: AdjacencyMatrix<Undirected> = AdjacencyMatrix::new(&m).unwrap();
        g.add_node(3);
        g.add_undirected_edge(1, 3);
        assert!(g.has_edge(1, 3));
        assert!(g.has_edge(3, 1));
        assert_eq!(g.undirected_size(), 3);
    }

    #[test]
    fn graph_remove_edge() {
        let m = vec![
            vec![0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0],
        ];
        let mut g: AdjacencyMatrix<Directed> = AdjacencyMatrix::new(&m).unwrap();
        g.remove_edge(1, 4);
        assert!(!g.has_edge(1, 4));
        assert!(!g.has_edge(4, 1));
        assert_eq!(g.size(), 4);
    }

    #[test]
    fn graph_duplicate_remove_edge() {
        let m = vec![
            vec![0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0],
        ];
        let mut g: AdjacencyMatrix<Directed> = AdjacencyMatrix::new(&m).unwrap();
        g.remove_edge(1, 4);
        g.remove_edge(1, 4);
        assert!(!g.has_edge(1, 4));
        assert!(!g.has_edge(4, 1));
        assert_eq!(g.size(), 4);
    }

    #[test]
    fn graph_remove_node() {
        let m = vec![
            vec![0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0],
        ];
        let mut g: AdjacencyMatrix<Directed> = AdjacencyMatrix::new(&m).unwrap();
        g.remove_node(2);
        assert_eq!(g.order(), 4);
        assert_eq!(g.size(), 2);
        assert!(!g.has_edge(3, 2));
        assert!(!g.has_edge(1, 2));
        assert!(!g.has_edge(4, 2));
    }

    #[test]
    fn graph_remove_node_and_add_new() {
        let m = vec![
            vec![0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0],
        ];
        let mut g: AdjacencyMatrix<Directed> = AdjacencyMatrix::new(&m).unwrap();

        g.remove_node(2);
        g.add_node(0);
        g.add_edge(2, 4);
        g.add_edge(2, 3);

        assert_eq!(g.order(), 5);
        assert_eq!(g.size(), 4);
        assert!(g.has_edge(1, 3));
        assert!(g.has_edge(2, 3));
        assert!(g.has_edge(2, 4));
        assert!(!g.has_edge(4, 2));
    }

    #[test]
    fn undirected_graph_remove_edge() {
        let m = vec![
            vec![0, 1, 0, 0, 0],
            vec![1, 0, 1, 0, 1],
            vec![0, 1, 0, 1, 1],
            vec![0, 0, 1, 0, 0],
            vec![0, 1, 1, 0, 0],
        ];
        let mut g: AdjacencyMatrix<Undirected> = AdjacencyMatrix::new(&m).unwrap();
        g.remove_undirected_edge(2, 4);
        assert!(!g.has_edge(2, 4));
        assert!(!g.has_edge(4, 2));
        assert_eq!(g.undirected_size(), 4);
    }

    #[test]
    fn undirected_graph_remove_node() {
        let m = vec![
            vec![0, 1, 0, 0, 0],
            vec![1, 0, 1, 0, 1],
            vec![0, 1, 0, 1, 1],
            vec![0, 0, 1, 0, 0],
            vec![0, 1, 1, 0, 0],
        ];
        let mut g: AdjacencyMatrix<Undirected> = AdjacencyMatrix::new(&m).unwrap();
        g.remove_node(4);
        assert_eq!(g.undirected_size(), 3);
        assert!(!g.has_edge(2, 4));
        assert!(!g.has_edge(1, 4));
    }

    #[test]
    fn node_degree_adjacency_matrix() {
        let m = vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]];
        let g: AdjacencyMatrix<Undirected> = AdjacencyMatrix::new(&m).unwrap();
        assert_eq!(g.undirected_node_degree(0), 2);
        assert_eq!(g.undirected_node_degree(1), 2);
        assert_eq!(g.undirected_node_degree(2), 2);
    }

    #[test]
    fn adjacency_matrix_order() {
        let m = vec![vec![0, 1, 1], vec![1, 0, 0], vec![1, 0, 0]];
        let g: AdjacencyMatrix<Undirected> = AdjacencyMatrix::new(&m).unwrap();
        assert_eq!(g.undirected_order(), 3);
    }
}
