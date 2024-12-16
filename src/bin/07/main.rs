pub mod equation;
pub mod operation;

fn main() {
    let input = std::fs::read_to_string("./files/07.txt").unwrap();

    println!("Day 7");

    println!("Part 1: {}", part1(&input))
}

pub fn part1(input: &str) -> u64 {
    crate::equation::parse_equations(input)
        .unwrap()
        .iter()
        .filter(|equation| equation.is_possible())
        .map(|equation| equation.test_value)
        .sum()
}
