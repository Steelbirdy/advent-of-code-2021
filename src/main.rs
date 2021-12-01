mod day01;
mod day02;

fn get_lines<T: std::str::FromStr>(file_path: &str) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    let buf = std::fs::read_to_string(file_path).expect("Could not find file");
    buf.lines()
        .map(|line| line.parse().expect("Failed to parse line"))
        .collect()
}

fn main() {
    let lines = get_lines("src/day02/input.txt");

    println!("Part 1: {}", day02::part1(lines.clone()));
    println!("Part 2: {}", day02::part2(lines));
}
