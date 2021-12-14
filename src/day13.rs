use std::collections::HashSet;

pub type Input = (HashSet<(u32, u32)>, Vec<(char, u32)>);

#[aoc_generator(day13)]
pub fn generator(input: &str) -> Input {
    let mut points = HashSet::new();
    let mut folds = Vec::new();

    let mut is_fold = false;

    for line in input.lines() {
        if line.is_empty() {
            is_fold = true;
            continue;
        }
        if is_fold {
            let (dir, v) = line["fold along ".len()..].split_once('=').unwrap();
            let dir = dir.chars().next().unwrap();
            let v = v.parse().unwrap();
            folds.push((dir, v));
        } else {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            points.insert((x, y));
        }
    }

    (points, folds)
}

fn fold(points: HashSet<(u32, u32)>, dir: char, v: u32) -> HashSet<(u32, u32)> {
    points
        .into_iter()
        .map(|(x, y)| {
            if dir == 'x' && x > v {
                (2 * v - x, y)
            } else if dir == 'y' && y > v {
                (x, 2 * v - y)
            } else {
                (x, y)
            }
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn part1(input: &Input) -> usize {
    let (points, folds) = input;
    let &(dir, v) = folds.first().unwrap();

    let points = points.clone();
    fold(points, dir, v).len()
}

#[aoc(day13, part2)]
pub fn part2(input: &Input) -> &'static str {
    let (points, folds) = input;

    let mut points = points.clone();

    for &(dir, v) in folds {
        points = fold(points, dir, v);
    }

    let &(max_x, _) = points.iter().max_by_key(|(x, _)| *x).unwrap();
    let &(_, max_y) = points.iter().max_by_key(|(_, y)| *y).unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            print!("{}", if points.contains(&(x, y)) { "#" } else { "." });
        }
        println!();
    }

    "check grid"
}
