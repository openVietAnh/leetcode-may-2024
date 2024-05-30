use problems::*;

pub mod problems;

fn main() {
    println!("{}", count_triplets(vec![2, 3, 1, 6, 7]));
    println!("{}", count_triplets(vec![1, 1, 1, 1, 1]));
}
