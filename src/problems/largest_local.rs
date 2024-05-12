pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![vec![0; grid.len() - 2]; grid.len() - 2];
    let dir: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for i in 1..grid.len() - 1 {
        for j in 1..grid.len() - 1 {
            result[i - 1][j - 1] = dir
                .clone()
                .into_iter()
                .map(|(a, b)| grid[(i as i32 + a) as usize][(j as i32 + b) as usize])
                .max()
                .unwrap()
        }
    }
    result
}
