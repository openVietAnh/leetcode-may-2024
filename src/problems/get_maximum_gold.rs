use std::{
    collections::{hash_map, HashSet},
    hash::{Hash, Hasher},
};

pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
    let mut stack = vec![];
    let mut result = 0;
    let dir: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != 0 {
                let mut hasher = hash_map::DefaultHasher::new();
                (i, j).hash(&mut hasher);
                stack.push((i, j, grid[i][j], HashSet::from([hasher.finish()])));
            }
        }
    }
    while stack.len() != 0 {
        let (x, y, value, path) = stack.pop().unwrap();
        result = std::cmp::max(result, value);
        for (i, j) in &dir {
            if i + x as i32 >= 0 && i + (x as i32) < grid.len() as i32 {
                if j + y as i32 >= 0 && j + (y as i32) < grid[0].len() as i32 {
                    let new_x = (i + x as i32) as usize;
                    let new_y = (j + y as i32) as usize;
                    let mut hasher = hash_map::DefaultHasher::new();
                    (new_x, new_y).hash(&mut hasher);
                    let new_node = hasher.finish();
                    if grid[new_x][new_y] != 0 && !path.contains(&new_node) {
                        let mut new_path = path.clone();
                        new_path.insert(new_node);
                        stack.push((new_x, new_y, value + grid[new_x][new_y], new_path));
                    }
                }
            }
        }
    }
    result
}
