pub type Input = i32;

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<Input> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &[Input]) -> Input {
    let highest = input.iter().copied().max().unwrap();
    (0..=highest)
        .map(|pos| input.iter().map(|&x| (x - pos).abs()).sum())
        .min()
        .unwrap()
}

fn sum_1_to_n(x: Input) -> Input {
    x * (x + 1) / 2
}

#[aoc(day7, part2)]
pub fn part2(input: &[Input]) -> Input {
    let highest = input.iter().copied().max().unwrap();
    (0..=highest)
        .map(|pos| input.iter().map(|&x| sum_1_to_n((x - pos).abs())).sum())
        .min()
        .unwrap()
}
