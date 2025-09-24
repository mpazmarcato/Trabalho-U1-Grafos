use crate::Graph;
use crate::graphs::{AdjacencyMatrix, IncidenceMatrix};

#[derive(Debug, Clone)]
pub struct AdjacencyList(pub Vec<Vec<usize>>);

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

    pub fn from_incidence_matrix(_matrix: &IncidenceMatrix) -> Self {
        todo!()
    }
}

impl Graph<usize> for AdjacencyList {
    fn order(&self) -> usize {
        self.0.len()
    }

    fn size(&self) -> usize {
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

    type Neighbors<'a> = Box<dyn Iterator<Item = usize> + 'a>;

    fn neighbors<'a>(&'a self, n: usize) -> Self::Neighbors<'a> {
        match self.0.get(n) {
            Some(edges) => Box::new(edges.iter().copied()),
            None => Box::new(std::iter::empty()),
        }
    }

    fn connected(&self) -> bool {
        for i in 0..self.order() {
            if self.dfs(i).count() != self.order() {
                return false;
            }
        }
        true
    }

    fn biparted(&self) -> bool {
        todo!()
    }

    #[allow(unreachable_code)]
    fn biconnected_components(&self) -> &[Vec<usize>] {
        todo!();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn connected_graph() {
        // Graph: 2 ── 0 ── 1
        // should be connected.
        assert!(AdjacencyList(vec![vec![1, 2], vec![0], vec![0]]).connected())
    }

    #[test]
    fn unconnected_graph() {
        // Graph: 2    0 ── 1
        // should be not connected.
        assert!(!AdjacencyList(vec![vec![1], vec![0], vec![]]).connected())
    }
}
