use crate::graphs::AdjacencyMatrix;
use crate::graphs::Graph;

#[derive(Debug, Clone)]
pub struct AdjacencyList(pub Vec<Vec<usize>>);

impl AdjacencyList {
    pub fn from_adjacency_matrix(matrix: &AdjacencyMatrix) -> Self {
        let mut adjacency_list = vec![Vec::new(); matrix.0.len()];
        for (i, row) in matrix.0.iter().enumerate() {
            adjacency_list[i].extend(
                row.iter().enumerate().filter_map(|(j, &val)| {
                    if val != 0 {
                        Some(j)
                    } else {
                        None
                    }
                }),
            );
        }
        AdjacencyList(adjacency_list)
    }
}

// impl Graph
impl Graph for AdjacencyList {
    fn nodes(&self) -> usize {
        self.0.len()
    }

    fn edges(&self) -> usize {
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
        if n < self.0.len() && m < self.0.len() && !self.0[n].contains(&m) {
            self.0[n].push(m);
        }
    }

    fn remove_edge(&mut self, n: usize, m: usize) {
        if n < self.0.len() {
            self.0[n].retain(|&x| x != m);
        }
    }

    fn neighbors(&self, n: usize) -> Vec<usize> {
        if n < self.0.len() {
            self.0[n].clone()
        } else {
            Vec::new()
        }
    }

    fn has_edge(&self, n: usize, m: usize) -> bool {
        n < self.0.len() && self.0[n].contains(&m)
    }
}
