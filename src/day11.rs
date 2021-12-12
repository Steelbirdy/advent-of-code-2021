use itertools::Itertools;
use std::collections::HashSet;

const COLS: usize = 10;
const ROWS: usize = 10;

type Grid<T> = [[T; COLS]; ROWS];

fn flash(grid: &mut Grid<u8>, flashed: &mut HashSet<(usize, usize)>, r: usize, c: usize) -> usize {
    flashed.insert((r, c));

    let mut count = 1;
    grid[r][c] = 0;

    for (dr, dc) in (-1..=1).cartesian_product(-1..=1) {
        if dr == dc && dr == 0 {
            continue;
        }

        let (nr, nc) = (r as isize + dr, c as isize + dc);
        if nr >= 0 && nr < ROWS as isize && nc >= 0 && nc < COLS as isize {
            let (nr, nc) = (nr as usize, nc as usize);
            grid[nr][nc] += 1;
            if grid[nr][nc] > 9 && !flashed.contains(&(nr, nc)) {
                count += flash(grid, flashed, nr, nc);
            }
        }
    }

    count
}

fn step(grid: &mut Grid<u8>, flashed: &mut HashSet<(usize, usize)>) -> usize {
    let mut count = 0;

    for line in grid.iter_mut() {
        for o in line {
            *o += 1;
        }
    }

    let mut any;
    loop {
        any = false;

        for (r, c) in (0..10).cartesian_product(0..10) {
            if grid[r][c] > 9 && !flashed.contains(&(r, c)) {
                count += flash(grid, flashed, r, c);
                any = true;
            }
        }

        if !any {
            break;
        }
    }

    for (r, c) in flashed.drain() {
        grid[r][c] = 0;
    }

    count
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    let mut grid = [[0; COLS]; ROWS];

    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.bytes().enumerate() {
            grid[r][c] = ch - b'0';
        }
    }

    let mut flashed = HashSet::with_capacity(100);

    (0..100).map(|_| step(&mut grid, &mut flashed)).sum()
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    let mut grid = [[0; COLS]; ROWS];

    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.bytes().enumerate() {
            grid[r][c] = ch - b'0';
        }
    }

    let mut flashed = HashSet::with_capacity(100);

    let mut count = 1;

    while step(&mut grid, &mut flashed) != 100 {
        count += 1;
    }

    count
}
