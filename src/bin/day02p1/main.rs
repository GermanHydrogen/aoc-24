use aoc_24::load_puzzle_input::load_puzzle_input;

fn solve(input: String) -> u64
{
    let rows = input.split("\n").collect::<Vec<&str>>();
    
    rows.iter().map(|row| row.split(" ").map(|x| x.parse().unwrap()).collect::<Vec<u64>>()).map(|row| check_safety(row) as u64).sum()
}

fn check_safety(levels: Vec<u64>) -> bool
{
    let first_iterator = levels.iter();
    let second_iterator = levels.iter().skip(1);
    if levels[1] > levels[0]
    {
        return second_iterator.zip(first_iterator).all(|(a, b)| a > b && a - b <= 3);
    }

    second_iterator.zip(first_iterator).all(|(a, b)| b > a && b - a <= 3)
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

        assert_eq!(result, 2);
    }
}