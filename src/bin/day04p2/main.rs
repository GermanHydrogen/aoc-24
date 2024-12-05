use aoc_24::load_puzzle_input::load_puzzle_input;
use aoc_24::vector::Vector;


fn solve(input: String) -> u64
{
    let matrix: Vec<Vec<String>> = input.split("\n").map(|x| x.split("").map(|x| x.to_string()).collect()).collect();
    let mut output: u64 = 0;
    
    for y in 0..matrix.len()
    {
        for x in 0..matrix[0].len()
        {
            output += find_xmas_at_index(&matrix, Vector::new(x as i64, y as i64));  
        }
    }
        
    output
}

fn find_xmas_at_index(matrix: &Vec<Vec<String>>, pos: Vector) -> u64
{
    let first = match accum_dir(matrix, pos, Vector::new(1, 1)) { 
        Some(x) => x,
        None => return 0
    };
    
    if first != "MAS".to_string() && first != "SAM"
    {
        return 0;
    }

    let second = match accum_dir(matrix, pos.add(&Vector::new(2, 0)), Vector::new(-1, 1)) {
        Some(x) => x,
        None => return 0
    };

    (second == "MAS" || second == "SAM") as u64
    
}

fn accum_dir(matrix: &Vec<Vec<String>>, pos: Vector, dir: Vector) -> Option<String>
{
    let mut output = String::new();
    
    for i in 0..3
    {
        match get_from_matrix(matrix, pos.add(&dir.multiply(i))) {
            Some(x) => output.push_str(&x),
            None => return None
        }
    }
    
    Some(output)
}

fn get_from_matrix(matrix: &Vec<Vec<String>>, pos: Vector) -> Option<String>
{
    if pos.y as usize >= matrix.len() || pos.y < 0
    {
        return None
    }

    if pos.x as usize >= matrix[0].len() || pos.x < 0
    {
        return None
    }

    Some(matrix[pos.y as usize][pos.x as usize].clone())
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

        assert_eq!(result, 9);
    }
}