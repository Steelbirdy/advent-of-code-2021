type Input = u32;

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<Input> {
    crate::parse_lines(input)
}

#[aoc(day1, part1)]
pub fn part1(input: &[Input]) -> usize {
    input.windows(2).filter(|w| w[0] < w[1]).count()
}

#[aoc(day1, part2)]
pub(crate) fn part2(input: &[Input]) -> usize {
    input.windows(4).filter(|w| w[0] < w[3]).count()
}
