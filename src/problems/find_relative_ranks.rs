pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut position: Vec<(usize, i32)> = score.clone().into_iter().enumerate().collect();
    position.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let mut result: Vec<String> = vec![String::from(""); score.len()];
    for (index, item) in position.iter().enumerate() {
        if index == 0 {
            result[item.0] = String::from("Gold Medal");
        } else if index == 1 {
            result[item.0] = String::from("Silver Medal");
        } else if index == 2 {
            result[item.0] = String::from("Bronze Medal");
        } else {
            result[item.0] = (index + 1).to_string();
        }
    }
    result
}