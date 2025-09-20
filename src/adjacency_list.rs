use crate::Graph;
use crate::graphs::AdjacencyMatrix;

#[derive(Debug, Clone)]
pub struct AdjacencyList(Vec<Vec<usize>>);

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
}

impl Graph<usize> for AdjacencyList {
    fn nodes(&self) -> usize {
        self.0.len()
    }

    fn edges(&self) -> usize {
        self.0.iter().map(|neighbors| neighbors.len()).sum()
    }

    // TODO: review the semantics of this.
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
            && let Some(edges) = self.0.get_mut(n)
            && !edges.contains(&m)
        {
            edges.push(m);
        }
    }

    fn remove_edge(&mut self, n: usize, m: usize) {
        if let Some(edges) = self.0.get_mut(n) {
            edges.retain(|&x| x != m);
        }
    }

    fn neighbors(&self, n: usize) -> Vec<usize> {
        if let Some(edges) = self.0.get(n) {
            edges.clone()
        } else {
            Vec::new()
        }
    }

    fn has_edge(&self, n: usize, m: usize) -> bool {
        if let Some(edges) = self.0.get(n) {
            edges.contains(&m)
        } else {
            false
        }
    }

    fn dfs(&self, start: usize) -> usize {
        fn component_size(g: &[Vec<usize>], n: usize, visited: &mut [bool]) -> Option<usize> {
            *visited.get_mut(n)? = true;
            let mut count = 1;
            for &m in g.get(n)? {
                if !*visited.get(m)? {
                    count += component_size(g, m, visited)?;
                }
            }
            Some(count)
        }
        component_size(&self.0, start, &mut vec![false; self.0.len()]).unwrap_or(0)
    }

    fn bfs(&self, start: usize) -> usize {
        _ = start;
        todo!()
    }

    fn node_degree(&self, n: usize) -> Option<usize> {
        Some(self.0.get(n)?.len())
    }

    fn connected(&self) -> bool {
        todo!()
    }

    fn biparted(&self) -> bool {
        todo!()
    }

    fn biconnected_components(&self) -> Vec<impl Graph<usize>> {
        todo!();
        vec![AdjacencyList(Vec::new())]
    }
}
