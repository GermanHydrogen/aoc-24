use aoc_24::load_puzzle_input::load_puzzle_input;

fn solve(input: String) -> u64
{
    0
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

        assert_eq!(result, 0);
    }
}