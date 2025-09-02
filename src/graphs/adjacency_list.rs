pub fn list_matrix(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = matrix.len();
    let mut adjacency_list: Vec<Vec<i32>> = vec![Vec::new(); n];

    for i in 0..n {
        for j in 0..n {
            if matrix[i][j] != 0 {
                adjacency_list[i].push(j as i32);
            }
        }
    }

    adjacency_list
}
