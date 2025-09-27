use std::{
    fs::File,
    io::{self, Write},
};

use crate::graphs::{AdjacencyList, IncidenceMatrix};
use crate::{Graph, UndirectedGraph};

#[derive(Debug)]
pub struct Node {
    value: usize,
    visited: bool,
    ancestor: Option<usize>,
}

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
        let mut stack: Vec<(usize, usize)> = Vec::new();

        for (idx_r, row) in self.0.iter().enumerate() {
            for (idx_c, col) in row.iter().enumerate() {
                if *col == 1 && !stack.contains(&(idx_r, idx_c)) && !stack.contains(&(idx_c, idx_r))
                {
                    stack.push((idx_r, idx_c));
                }
            }
        }

        stack.len()
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

impl UndirectedGraph<usize> for AdjacencyMatrix {}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

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
        assert!(undirected_m.size() == 9);
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
        assert!(directed_m.size() == 9);
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
        assert_eq!(original_graph.size(), underlying_graph.size());
    }

    #[test]
    fn graph_add_new_node() {
        // Graph: 0 -> 2 <- 1
        let mut m = AdjacencyMatrix(vec![vec![0, 0, 1], vec![0, 0, 1], vec![0, 0, 0]]);
        m.add_node(3);
        // Graph: 0 -> 2 <- 1  3
        assert!(m.order() == 4);
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
        assert!(m.size() == 3);
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
        assert!(m.size() == 4);
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
        assert!(m.size() == 3);
        assert!(!m.has_edge(2, 4));
        assert!(!m.has_edge(1, 4));
    }
}
