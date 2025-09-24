use std::{
    fs::File,
    io::{self, Write},
};

use crate::{
    Graph,
    graphs::{AdjacencyList, IncidenceMatrix},
};
// TODO: remove this!
#[derive(Debug)]
pub struct Node {
    value: usize,
    visited: bool,
    ancestor: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct AdjacencyMatrix(pub Vec<Vec<usize>>);

impl Graph<usize> for AdjacencyMatrix {
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
        // Catch edges loops
        if n == m {
            match self.0.get_mut(n) {
                Some(edges) => edges[n] = 1,
                None => todo!(),
            }
        } else {
            let (a, b) = if n < m { (n, m) } else { (m, n) }; // Re-order
            let (left, right) = self.0.split_at_mut(b);

            match (left.get_mut(a), right.get_mut(0)) {
                (None, None) => panic!(), // TODO: should treat?
                (None, Some(_)) => panic!(),
                (Some(_), None) => panic!(),
                (Some(n_edges), Some(m_edges)) => {
                    n_edges[b] = 1;
                    m_edges[a] = 1;
                }
            }
        }
    }

    fn order(&self) -> usize {
        self.0.len()
    }

    fn size(&self) -> usize {
        todo!()
    }

    fn remove_node(&mut self, n: &usize) {
        todo!()
    }

    fn remove_edge(&mut self, n: &usize, m: &usize) {
        todo!()
    }

    fn connected(&self) -> bool {
        todo!()
    }

    fn biconnected_components(&self) -> &[Vec<&usize>] {
        todo!()
    }

    fn biparted(&self) -> bool {
        todo!()
    }

    type Neighbors<'a>
        = std::slice::Iter<'a, usize>
    where
        Self: 'a,
        usize: 'a;

    fn neighbors<'a>(&'a self, n: &usize) -> Self::Neighbors<'a> {
        todo!()
    }
}

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

    pub fn dfs(&self) -> i32 {
        self.dfs_from_node(0)
    }

    pub fn dfs_from_node(&self, start: usize) -> i32 {
        let mut vertices: Vec<Node> = (0..self.0.len())
            .map(|i| Node {
                value: i,
                visited: false,
                ancestor: None,
            })
            .collect();

        let mut stack: Vec<usize> = Vec::new();
        let initial: usize = if start < self.0.len() { start } else { 0 };

        stack.push(initial);
        vertices[initial].visited = true;
        let mut visited_count = 1;

        while let Some(row) = stack.last().copied() {
            let unvisited: Option<usize> = self.0[row]
                .iter()
                .enumerate()
                .find(|&(idx, &val)| val == 1 && !vertices[idx].visited)
                .map(|(i, _)| i);

            if let Some(node) = unvisited {
                vertices[node].visited = true;
                vertices[node].ancestor = Some(row);
                stack.push(node);
                visited_count += 1;
            } else {
                stack.pop();
            }
        }
        visited_count
    }

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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn add_new_node() {
        // Graph: 0 - 2 - 1
        let mut m = AdjacencyMatrix(vec![vec![0, 0, 1], vec![0, 0, 1], vec![1, 1, 0]]);
        m.add_node(3);
        // Graph: 0 - 2 - 1  3
        assert!(m.order() == 4);
    }

    #[test]
    fn add_new_node_and_edge() {
        // Graph: 0 - 2 - 1
        let mut m = AdjacencyMatrix(vec![vec![0, 0, 1], vec![0, 0, 1], vec![1, 1, 0]]);
        m.add_node(3);
        m.add_edge(1, 3);
        // Graph: 0 - 2 - 1 - 3
        // assert!(m.has_edge(&1, &3)); TODO: use when neighbors is implemented..
    }
}
