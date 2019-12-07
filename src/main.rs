mod probs;
pub mod utils;

fn main() {
    println!("Hello, world!");
    println!("prob01 part1: {}", probs::p01::resolve());
    println!("prob01 part2: {}", probs::p01p2::resolve());
    println!("prob02 part1: {}", probs::p02::resolve());
    println!("prob02 part2: {:?}", probs::p02p2::resolve());
}
