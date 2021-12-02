#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<u32> {
    crate::parse_lines(input)
}

#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> usize {
    input.windows(2).filter(|w| w[0] < w[1]).count()
}

#[aoc(day1, part2)]
pub(crate) fn part2(input: &[u32]) -> usize {
    input.windows(4).filter(|w| w[0] < w[3]).count()
}
