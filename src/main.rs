use crate::problems::reverse_prefix::reverse_prefix;

mod problems;

fn main() {
    println!("{}", reverse_prefix(String::from("abcdefd"), 'd'));
}
