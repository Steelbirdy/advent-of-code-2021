type Input = u32;
type Output = usize;

pub(crate) fn part1(lines: Vec<Input>) -> Output {
    lines
        .windows(2)
        .filter(|w| matches!(w, &&[a, b] if a < b))
        .count()
}

pub(crate) fn part2(lines: Vec<Input>) -> Output {
    lines
        .windows(3)
        .map(|w| w.iter().copied().sum::<u32>())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|w| matches!(w, &&[a, b] if a < b))
        .count()
}
