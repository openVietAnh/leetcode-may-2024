pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let mut frac = vec![];
    for i in 0..arr.len() - 1 {
        for j in i + 1..arr.len() {
            frac.push((arr[i] as f64 / arr[j] as f64, arr[i], arr[j]));
        }
    }
    frac.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    vec![frac[k as usize - 1].1, frac[k as usize - 1].2]
}
