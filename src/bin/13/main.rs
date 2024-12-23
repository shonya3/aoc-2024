pub mod part1;

fn main() {
    let input = std::fs::read_to_string("./files/13.txt").unwrap();

    println!("Day 13");

    println!("Part 1: {}", part1(&input));
}

pub fn part1(input: &str) -> usize {
    part1::parse_input(input)
        .unwrap()
        .iter()
        .filter_map(|group| group.find_solution())
        .sum()
}

#[cfg(test)]
mod tests {

    const EXAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    pub fn part1() {
        assert_eq!(480, super::part1(EXAMPLE));
    }
}
