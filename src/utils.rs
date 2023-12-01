pub fn print_header(value: &str, equals_count: usize) {
    let border = "=".repeat(equals_count);
    println!("{border} {value} {border}");
}

pub fn advent_header() {
    print_header("Advent of Code 2023", 10);
    println!();
}

pub fn day_header(day: u8) {
    println!();
    print_header(format!("Day {day}").as_str(), 5);
}

pub fn part_header(part: u8) {
    println!();
    print_header(format!("Part {part}").as_str(), 3);
}
