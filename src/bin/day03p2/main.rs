use regex::Regex;
use aoc_24::load_puzzle_input::load_puzzle_input;


struct State
{
    enabled: bool
}

impl State
{
    fn new() -> Self
    {
        State { enabled: true }
    }
    
    fn process(&mut self, input: &str) -> u64
    {
        if input == "do()"
        {
            self.enabled = true;
            return 0;
        }else if input == "don't()" { 
            self.enabled = false;
            return 0;
        }
        
        if !self.enabled
        {
            return 0;
        }
        
        let re = Regex::new(r"mul\((\d|\d\d|\d\d\d),(\d|\d\d|\d\d\d)\)").unwrap();
        
        let (_, [a, b]) = re.captures(input).unwrap().extract();
        
        a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap()
    }
}


fn solve(input: String) -> u64
{
    let re = Regex::new(r"(mul\((?:\d|\d\d|\d\d\d),(?:\d|\d\d|\d\d\d)\))|(do\(\))|(don't\(\))").unwrap();
    let mut state = State::new();
    re.captures_iter(&input).map(|c| c.extract()).map(|(_, [part]) | state.process(part)).sum()
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

        assert_eq!(result, 48);
    }
}