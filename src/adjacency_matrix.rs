use std::{fs::File, io, io::Write};

use crate::Graph;
use crate::graphs::{AdjacencyList, IncidenceMatrix};

// FIXME: ideally the struct field should be private.
#[derive(Debug, Clone)]
pub struct AdjacencyMatrix(pub Vec<Vec<i32>>);

#[derive(Debug)]
pub struct Node {
    value: usize,
    visited: bool,
    ancestor: Option<usize>,
}

impl AdjacencyMatrix {
    pub fn from_adjacency_list(_list: &AdjacencyList) -> Self {
        let n = _list.0.len();
        let mut adjacency_matrix: Vec<Vec<i32>> = vec![vec![0; n]; n];

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

    pub fn bfs(&self) -> i32 {
        self.bfs_from_node(0)
    }

    pub fn bfs_from_node(&self, start: usize) -> i32 {
        let mut vertices: Vec<Node> = (0..self.0.len())
            .map(|i| Node {
                value: i,
                visited: false,
                ancestor: None,
            })
            .collect();

        let mut queue: std::collections::VecDeque<usize> = std::collections::VecDeque::new();
        let initial: usize = if start < self.0.len() { start } else { 0 };

        queue.push_back(initial);
        vertices[initial].visited = true;
        let mut visited_count = 1;

        while let Some(node_idx) = queue.pop_front() {
            for (i, &val) in self.0[node_idx].iter().enumerate() {
                if val == 1 && !vertices[i].visited {
                    vertices[i].visited = true;
                    vertices[i].ancestor = Some(node_idx);
                    queue.push_back(i);
                    visited_count += 1;
                }
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

impl Graph<usize> for AdjacencyMatrix {
    fn order(&self) -> usize {
        self.0.len()
    }

    fn size(&self) -> usize {
        todo!()
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

    type Neighbors<'a> = Box<dyn Iterator<Item = usize> + 'a>;
    fn neighbors<'a>(&'a self, n: usize) -> Self::Neighbors<'a> {
        match self.0.get(n) {
            Some(row) => Box::new(
                row.iter()
                    .enumerate()
                    .filter_map(|(i, &weight)| if weight != 0 { Some(i) } else { None }),
            ),
            None => Box::new(std::iter::empty()),
        }
    }

    fn connected(&self) -> bool {
        todo!()
    }

    fn biconnected_components(&self) -> &[Vec<usize>] {
        todo!()
    }

    fn biparted(&self) -> bool {
        todo!()
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
}