use std::collections::HashMap;

pub fn count_triplets(arr: Vec<i32>) -> i32 {
    let mut prefix: HashMap<i32, Vec<usize>> = HashMap::new();
    let mut result: i32 = 0;
    let mut prefix_xor = 0;
    for (index, item) in arr.into_iter().enumerate() {
        prefix_xor ^= item;
        if prefix_xor == 0 {
            result += index as i32;
        }
        if prefix.contains_key(&prefix_xor) {
            for item in prefix.get(&prefix_xor).unwrap() {
                result += (index - item - 1) as i32;
            }
        }
        prefix
            .entry(prefix_xor)
            .and_modify(|arr| arr.push(index))
            .or_insert(vec![index]);
    }
    result
}
