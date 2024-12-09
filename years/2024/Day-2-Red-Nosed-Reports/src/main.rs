fn main() {
    let start_time = std::time::Instant::now();
    let input = include_bytes!("../input.txt");
    let reports = part1::RedNoseReactorReport::construct_reports(input);
    let safe_reports_part_one = reports.iter().filter(|r| r.is_safe_part_one()).count();
    let elapsed = start_time.elapsed();
    println!("{safe_reports_part_one} safe reports in {elapsed:?}");

    let start_time = std::time::Instant::now();
    let safe_reports_part_two = reports.iter().filter(|r| r.is_safe_part_two()).count();
    let elapsed = start_time.elapsed();
    println!("{safe_reports_part_two} safe reports in {elapsed:?}");
}

mod part1;
mod part2;
