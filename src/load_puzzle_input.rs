use std::fs;

pub fn load_puzzle_input(module_name: &str, test: bool) -> String
{
    let day = get_puzzle_day_from_module(module_name);
    return load_puzzle_file(day, test);
}

fn get_puzzle_day_from_module(module_name: &str) -> String
{
    let module = module_name.split("::").next().unwrap();
    return module.to_string();
}

fn load_puzzle_file(day: String, test: bool) -> String
{
    let mut test_prefix = "";
    if test
    {
        test_prefix = "test_";
    }
    let path = format!("puzzle_input/{}/{}input.txt", day, test_prefix);
    let puzzle_input = fs::read_to_string(path).expect("Puzzle input missing!");

    return puzzle_input
}