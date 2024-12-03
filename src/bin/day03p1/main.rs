use regex::Regex;
use aoc_24::load_puzzle_input::load_puzzle_input;


fn solve(input: String) -> u64
{
    let re = Regex::new(r"mul\((\d|\d\d|\d\d\d),(\d|\d\d|\d\d\d)\)").unwrap();
    
    re.captures_iter(&input).map(|c| c.extract()).map(|(_, [a, b]) | a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap()).sum()
}

fn main() {
    let puzzle_input = load_puzzle_input(module_path!(), false);
    let result = solve(puzzle_input);
    println!("{}", result)
}


#[cfg(test)]
mod tests {
    use std::time::Instant;
    use aoc_24::load_puzzle_input::{load_puzzle_input};
    use crate::solve;

    #[test]
    fn test_run() {
        let puzzle_input = load_puzzle_input(module_path!(), true);

        let now = Instant::now();
        
        let result = solve(puzzle_input);
        
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

        assert_eq!(result, 161);
    }
}