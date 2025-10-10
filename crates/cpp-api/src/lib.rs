use graphs_algorithms::graphs::AdjacencyMatrix;
use graphs_algorithms::{Direction, Graph};

unsafe extern "C" {
    fn mk_adjacency_list(node_amt: usize) -> *mut std::ffi::c_void;
    fn add_edge_unchecked(graph: *mut std::ffi::c_void, n: usize, m: usize);
    fn dfs(graph: *mut std::ffi::c_void, start: usize);
}

pub struct AdjacencyListCpp {
    ptr: *mut std::ffi::c_void,
}

impl AdjacencyListCpp {
    #[inline]
    pub fn new(node_amt: usize) -> Self {
        let ptr = unsafe { mk_adjacency_list(node_amt) };
        Self { ptr }
    }

    pub fn from_adjacency_matrix<D: Direction>(matrix: &AdjacencyMatrix<D>) -> Self {
        let adj_list = Self::new(matrix.order());

        for (i, row) in matrix.data().iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                if val == 1 {
                    adj_list.add_edge_unchecked(i, j);
                }
            }
        }
        adj_list
    }

    #[inline]
    pub fn add_edge_unchecked(&self, n: usize, m: usize) {
        unsafe { add_edge_unchecked(self.ptr, n, m) }
    }

    #[inline]
    pub fn dfs(&self, start: usize) {
        unsafe { dfs(self.ptr, start) }
    }
}
