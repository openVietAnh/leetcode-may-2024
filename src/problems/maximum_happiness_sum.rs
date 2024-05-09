pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
    happiness.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let mut result: i64 = 0;
    for i in 0..k {
        result += std::cmp::max(happiness[i as usize] - i, 0) as i64;
    }
    result
}