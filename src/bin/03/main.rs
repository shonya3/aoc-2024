mod part_1;

fn main() {
    let input = std::fs::read_to_string("./files/03.txt").unwrap();

    println!("Part 1 sum: {}", part_1::calc_input_muls(&input));
}
