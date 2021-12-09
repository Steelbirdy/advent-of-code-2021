use parse_display::{Display, FromStr};

type Input = [Points];

#[derive(Copy, Clone, Display, FromStr)]
#[display("{0},{1} -> {2},{3}")]
pub struct Points(usize, usize, usize, usize);

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<Points> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &Input) -> usize {
    let input = input
        .iter()
        .copied()
        .filter(|&Points(x1, y1, x2, y2)| x1 == x2 || y1 == y2);

    let mut grid: Vec<[u8; 1000]> = Vec::with_capacity(1000);
    grid.resize_with(1000, || [0; 1000]);

    for Points(x1, y1, x2, y2) in input {
        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                grid[x1][y] += 1;
            }
        } else {
            for x in grid.iter_mut().take(x1.max(x2) + 1).skip(x1.min(x2)) {
                x[y1] += 1;
            }
        }
    }

    grid.into_iter().flatten().filter(|&x| x > 1).count()
}

#[aoc(day5, part2)]
pub fn part2(input: &Input) -> usize {
    let input = input.iter().copied().filter(|&Points(x1, y1, x2, y2)| {
        x1 == x2 || y1 == y2 || y2 as isize - y1 as isize == x2 as isize - x1 as isize
    });

    let mut grid: Vec<[u8; 1000]> = Vec::with_capacity(1000);
    grid.resize_with(1000, || [0; 1000]);

    for Points(x1, y1, x2, y2) in input {
        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                grid[x1][y] += 1;
            }
        } else if y1 == y2 {
            for x in grid.iter_mut().take(x1.max(x2) + 1).skip(x1.min(x2)) {
                x[y1] += 1;
            }
        } else {
            for d in 0..=(x2 as isize - x1 as isize).abs() as usize {
                let x = if x2 > x1 { x1 + d } else { x1 - d };
                let y = if y2 > y1 { y1 + d } else { y1 - d };
                grid[x][y] += 1;
            }
        }
    }

    grid.into_iter().flatten().filter(|&x| x > 1).count()
}
