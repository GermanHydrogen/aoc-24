use std::collections::{HashMap, HashSet};
use aoc_24::load_puzzle_input::load_puzzle_input;

fn solve(input: String) -> u64
{
    let rows: Vec<&str> = input.split("\n").collect();
    let entries: Vec<Vec<&str>> = rows.iter().map(|x| x.split("   ").collect()).collect();
    
    let first_column = entries.iter().map(|row| row[0].parse().unwrap()).collect::<Vec<i64>>();
    let second_column = entries.iter().map(|row| row[1].parse().unwrap()).collect::<Vec<i64>>();
    
    let mut abundances: HashMap<i64, i64> = HashMap::new();
    
    for number in second_column.iter()
    {
        abundances.entry(*number).and_modify(|y| *y += 1).or_insert(1);
    }

    first_column.iter().map(|x| *x * *abundances.entry(*x).or_default()).sum::<i64>() as u64   
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

        assert_eq!(result, 31);
    }
}