use std::collections::HashSet;

pub fn find_max_k(nums: Vec<i32>) -> i32 {
    let mut result = -1;
    let mut s: HashSet<i32> = HashSet::new();
    for item in nums {
        s.insert(item);
        if item < 0 && s.contains(&(-item)) && -item > result {
            result = -item;
        }
        if item > 0 && s.contains(&(-item)) && item > result {
            result = item;
        }
    }
    result
}
