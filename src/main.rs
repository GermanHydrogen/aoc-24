use std::env;
use crate::load_puzzle_input::load_puzzle_input;

mod load_puzzle_input;
mod vector;


fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{}", args[0]);

    println!("{}", load_puzzle_input("day_template", true));
}
