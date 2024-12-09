use part1::part_one;
use part2::part_two;

fn main() {
    let input = include_bytes!("../input.txt");

    let start = std::time::Instant::now();
    let part_one_result = part_one(input);
    let elapsed = start.elapsed();
    println!("Part One {part_one_result} in {elapsed:?}");

    let start = std::time::Instant::now();
    let part_two_result = part_two(input);
    let elapsed = start.elapsed();
    println!("Part Two {part_two_result} in {elapsed:?}");
}

mod part1;
mod part2;
