use aoc_24::load_puzzle_input::load_puzzle_input;

fn solve(input: String) -> i64
{
    let rows = input.split("\n").collect::<Vec<&str>>();
    
    rows.iter().map(|row| row.split(" ").map(|x| x.parse().unwrap()).collect::<Vec<i64>>()).map(|row| check_levels(row, true) as i64).sum()
}


fn check_levels(levels: Vec<i64>, flag: bool) -> bool
{

    let checker = if levels[levels.len() - 1] > levels[0]
    {
        is_safe
    }else {
        is_flipped_safe
    };

    for i in 0..levels.len()-1
    {
        if !checker(levels[i], levels[i+1])
        {
            if flag
            {
                if !check_without(&levels, i)
                {
                    return false;
                }
            }else 
            { 
                return false; 
            }
            
        }
    }

    true
}

fn check_without(levels: &Vec<i64>, i: usize) -> bool
{
    let without_item: Vec<i64> = [&levels.as_slice()[0..i], &levels.as_slice()[i+1..levels.len()]].concat();
    if check_levels(without_item, false)
    {
        return true;
    }
    let without_next: Vec<i64> = [&levels.as_slice()[0..i+1], &levels.as_slice()[i+2..levels.len()]].concat();
    
    check_levels(without_next, false)
}

fn is_safe(first: i64, second: i64) -> bool
{
    second > first && second - first <= 3 && second - first >= 0
}

fn is_flipped_safe(second: i64, first: i64) -> bool
{
    second > first && second - first <= 3 && second - first >= 0
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
    use crate::{check_levels, solve};

    #[test]
    fn test_run() {
        let puzzle_input = load_puzzle_input(module_path!(), true);

        let now = Instant::now();
        
        let result = solve(puzzle_input);
        
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

        assert_eq!(result, 4);
    }
    
    #[test]
    fn test_solve()
    {
        let input: Vec<i64> = vec![63, 66, 65, 67, 69, 69];
        
        assert_eq!(check_levels(input, false), false)
    }
}