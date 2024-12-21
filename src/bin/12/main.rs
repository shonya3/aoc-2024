use garden::Garden;

mod garden;

fn main() {
    let input = std::fs::read_to_string("./files/12.txt").unwrap();

    println!("Day 12");

    println!("Part 1: {}", input.parse::<Garden>().unwrap().fence_price());
}
