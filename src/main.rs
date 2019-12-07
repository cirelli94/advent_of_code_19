mod probs;
pub mod utils;

fn main() {
    //println!("Hello, world!");
    //println!("prob01 part1: {}", prob01::resolve());
    //println!("prob01 part2: {}", prob01part2::resolve());
    //println!("prob02 part1: {}", probs::p02::resolve());

    let (noun, verb) = probs::p02p2::resolve();
    println!("prob02 part2: {:?}", 100 * noun + verb);
}
