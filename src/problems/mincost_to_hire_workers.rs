use std::collections::BinaryHeap;

pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
    let mut zip: Vec<(usize, f64)> = vec![];
    for i in 0..quality.len() {
        zip.push((i, wage[i] as f64 / quality[i] as f64));
    }
    zip.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let mut max_rate = 0.0;
    let mut quality_sum = 0;
    let mut heap = BinaryHeap::new();
    for i in 0..k {
        heap.push(quality[zip[i as usize].0]);
        if zip[i as usize].1 > max_rate {
            max_rate = zip[i as usize].1;
        }
        quality_sum += quality[zip[i as usize].0];
    }
    let mut result = max_rate * quality_sum as f64;
    for i in k as usize..wage.len() {
        if zip[i].1 > max_rate {
            max_rate = zip[i].1;
        }
        quality_sum -= heap.pop().unwrap();
        quality_sum += quality[zip[i].0];
        heap.push(quality[zip[i].0]);
        if quality_sum as f64 * max_rate < result {
            result = quality_sum as f64 * max_rate;
        }
    }
    result
}
