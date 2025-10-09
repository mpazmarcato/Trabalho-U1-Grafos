use crate::Graph;
use crate::graph::{Directed, Direction, FromGraph, GraphError, Undirected};
use crate::graphs::{AdjacencyList, AdjacencyMatrix};
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Clone)]
pub struct IncidenceMatrix<D: Direction> {
    data: Vec<Vec<i32>>,
    _marker: PhantomData<D>,
}

impl<D: Direction> Graph<usize> for IncidenceMatrix<D> {
    fn node_degree(&self, vertex: usize) -> usize {
        if self.data.is_empty() || vertex >= self.data[0].len() {
            return 0;
        }

        self.data.iter().filter(|row| row[vertex] != 0).count() / 2
    }

    fn order(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data[0].len()
        }
    }

    fn size(&self) -> usize {
        self.data.len() / 2
    }

    fn new(data: &Vec<Vec<i32>>) -> Result<Self, GraphError> {
        if data.is_empty() {
            return Ok(Self {
                data: data.clone(),
                _marker: PhantomData,
            });
        }

        let order = data[0].len();

        for row in data {
            if row.len() != order {
                return Err(GraphError::Dimensions);
            }
        }

        if D::is_directed() {
            for e in data {
                if let Some(&weight) = e.iter().find(|&&x| x != 0) {
                    // his oposite should be on the line
                    let has_correspondent = e.iter().any(|&x| x == -weight);

                    if !has_correspondent {
                        return Err(GraphError::InvalidLine(e.clone()));
                    }
                } else {
                    return Err(GraphError::InvalidLine(e.clone()));
                }
            }
        } else {
            // just to nodes by edge
            for e in data {
                let non_zero: Vec<&i32> = e.iter().filter(|&&x| x != 0).collect();
                if non_zero.len() != 2 || *non_zero[0] != *non_zero[1] {
                    return Err(GraphError::InvalidLine(e.clone()));
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

    fn add_node(&mut self, n: usize) {
        todo!()
    }

    fn remove_node(&mut self, n: usize) {
        todo!()
    }

    fn add_edge(&mut self, n: usize, m: usize) {
        todo!()
    }

    fn remove_edge(&mut self, n: usize, m: usize) {
        todo!()
    }

    type Neighbors<'a>
        = std::vec::IntoIter<usize>
    where
        D: 'a;

    fn neighbors<'a>(&'a self, n: usize) -> Self::Neighbors<'a> {
        todo!()
    }

    fn biparted(&self) -> bool {
        todo!()
    }

    fn underlying_graph(&self) -> Self {
        todo!()
    }

    fn has_edge(&self, n: usize, m: usize) -> bool {
        self.neighbors(n).any(|neighbor| neighbor == m)
    }

    fn dfs(&self, start: usize) -> crate::graph::DfsIter<'_, usize, Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn bfs(&self, start: usize) -> crate::graph::BfsIter<'_, usize, Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn classify_edges(&self, start: usize) -> crate::graph::DfsEdgesIter<'_, usize, Self>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl<D: Direction> FromGraph<usize, AdjacencyList<D>> for IncidenceMatrix<D> {
    fn from_graph(g: &AdjacencyList<D>) -> Self {
        let mut incidence_matrix: Vec<Vec<i32>> = Vec::new();

        for (c_out, v) in g.data().iter().enumerate() {
            for &c_in in v {
                let mut edge = vec![0; g.order()];

                if D::is_directed() {
                    edge[c_out] = -1;
                    edge[c_in as usize] = 1;
                } else {
                    edge[c_out] = 1;
                    edge[c_in as usize] = 1;
                }

                if !incidence_matrix.iter().any(|e| *e == edge) {
                    incidence_matrix.push(edge);
                }
            }
        }

        Self {
            data: incidence_matrix,
            _marker: PhantomData,
        }
    }
}

impl<D: Direction> FromGraph<usize, AdjacencyMatrix<D>> for IncidenceMatrix<D> {
    fn from_graph(g: &AdjacencyMatrix<D>) -> Self {
        let mut incidence_matrix: Vec<Vec<i32>> = vec![];

        for i in 0..g.data().len() {
            for j in 0..g.data().len() {
                if g.data()[i][j] != 0 {
                    let mut edge = vec![0; g.order()];

                    if D::is_directed() {
                        edge[i] = -g.data()[i][j];
                        edge[j] = g.data()[i][j];
                    } else {
                        edge[i] = g.data()[i][j];
                        edge[j] = g.data()[i][j];
                    }

                    incidence_matrix.push(edge);
                }
            }
        }

        Self::new(&incidence_matrix).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;

    #[test]
    fn incidence_matrix_directed_graph_new() {
        let incidence_matrix = vec![
            vec![-6, 0, 6, 0, 0, 0],
            vec![-1, 0, 0, 0, 1, 0],
            vec![0, -2, 0, 0, 0, 2],
            vec![0, 0, -11, 0, 0, 11],
            vec![0, 0, 8, -8, 0, 0],
            vec![0, 5, 0, 0, -5, 0],
            vec![0, 0, 0, 9, 0, -9],
            vec![0, 0, 0, 0, 3, -3],
        ];

        let new = IncidenceMatrix::<Directed>::new(&incidence_matrix);
        let constructor = IncidenceMatrix::<Directed> {
            data: incidence_matrix,
            _marker: PhantomData,
        };

        assert_eq!(new, Ok(constructor));
    }

    #[test]
    fn incidence_matrix_undirected_graph_new_error() {
        let weighted = vec![
            vec![-6, 0, 6, 0, 0, 0],
            vec![-1, 0, 0, 0, 1, 0],
            vec![0, -2, 0, 0, 0, 2],
            vec![0, 0, -11, 0, 0, 11],
            vec![0, 0, 8, -8, 0, 0],
            vec![0, 5, 0, 0, -5, 0],
            vec![0, 0, 0, 9, 0, -9],
            vec![0, 0, 0, 0, 3, -3],
        ];

        let graph = IncidenceMatrix::<Undirected>::new(&weighted);
        // trying to declare a undirected graph using a directed one as the content
        assert_eq!(graph, Err(GraphError::InvalidLine(weighted[0].clone())));
    }

    #[test]
    fn incidence_matrix_graph_new_dimensions_error() {
        let incidence_matrix = vec![
            vec![-6, 0, 6, 0, 0, 0],
            vec![-1, 0, 0, 0, 1, 0],
            vec![0, -2, 0, 0, 0, 2],
            vec![0, 0, -11, 0, 0], // edge missing node
            vec![0, 0, 8, -8, 0, 0],
            vec![0, 5, 0, 0, -5, 0],
            vec![0, 0, 0, 9, 0, -9],
            vec![0, 0, 0, 0, 3, -3],
        ];

        let graph = IncidenceMatrix::<Directed>::new(&incidence_matrix);

        assert_eq!(graph, Err(GraphError::Dimensions));
    }

    #[test]
    fn incidence_matrix_graph_new_invalid_line_error() {
        let incidence_matrix = vec![
            vec![-6, 0, 5, 0, 0, 0], // unmatching weights on edge
            vec![-1, 0, 0, 0, 1, 0],
            vec![0, -2, 0, 0, 0, 2],
            vec![0, 0, -11, 0, 0, 11],
            vec![0, 0, 8, -8, 0, 0],
            vec![0, 5, 0, 0, -5, 0],
            vec![0, 0, 0, 9, 0, -9],
            vec![0, 0, 0, 0, 3, -3],
        ];

        let graph = IncidenceMatrix::<Directed>::new(&incidence_matrix);

        assert_eq!(
            graph,
            Err(GraphError::InvalidLine(incidence_matrix[0].clone()))
        );
    }

    #[test]
    fn node_degree_incidence_matrix() {
        // Graph: 0 ── 1
        //        │
        //        2
        let g = vec![vec![0, 1, 1], vec![1, 0, 0], vec![1, 0, 0]];
        let matrix: AdjacencyMatrix<Undirected> = AdjacencyMatrix::new(&g).unwrap();

        let incidence = IncidenceMatrix::from_graph(&matrix);

        assert_eq!(incidence.node_degree(0), 2);
        assert_eq!(incidence.node_degree(1), 1);
        assert_eq!(incidence.node_degree(2), 1);
    }

    #[test]
    fn test_order_incidence_matrix() {
        // Graph: 0 ── 1 ── 2
        let g = vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 0]];
        let adj: AdjacencyMatrix<Undirected> = AdjacencyMatrix::new(&g).unwrap();

        let inc = IncidenceMatrix::from_graph(&adj);

        assert_eq!(inc.order(), 3);
    }

    #[test]
    fn test_size_incidence_matrix() {
        // Graph: 0 ── 1 ── 2
        let g = vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 0]];
        let adj: AdjacencyMatrix<Undirected> = AdjacencyMatrix::new(&g).unwrap();

        let inc = IncidenceMatrix::from_graph(&adj);

        assert_eq!(inc.size(), 2);
    }

    #[test]
    fn from_adjacencylist_digraph() {
        //    (0)      ->(1)
        //       \a₁  /a₃  \a₂
        //        ->(3)     ->(2)
        //       /a₄
        //     (4)
        let g = vec![vec![3], vec![2], vec![], vec![1], vec![3]];
        let adj: AdjacencyList<Directed> = AdjacencyList::new(&g).unwrap();

        let answer: Vec<Vec<i32>> = vec![
            vec![-1, 0, 0, 1, 0],
            vec![0, -1, 1, 0, 0],
            vec![0, 1, 0, -1, 0],
            vec![0, 0, 0, 1, -1],
        ];

        let incidence_matrix = IncidenceMatrix::from_graph(&adj);

        assert_eq!(*incidence_matrix.data(), answer);
    }

    #[test]
    fn from_adjacencylist_graph() {
        //    (0)      --(1)
        //       \a₁  /a₃  \a₂
        //        --(3)     --(2)
        //       /a₄
        //     (4)
        let g = vec![vec![3], vec![2, 3], vec![1], vec![0, 1, 4], vec![3]];
        let adj: AdjacencyList<Undirected> = AdjacencyList::new(&g).unwrap();

        let answer: Vec<Vec<i32>> = vec![
            vec![1, 0, 0, 1, 0],
            vec![0, 1, 1, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 0, 0, 1, 1],
        ];

        let incidence_matrix = IncidenceMatrix::from_graph(&adj);

        assert_eq!(*incidence_matrix.data(), answer);
    }
}
