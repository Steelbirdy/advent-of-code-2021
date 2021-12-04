type Board = [[(u64, bool); 5]; 5];

#[aoc_generator(day4)]
pub fn generator(input: &str) -> (Vec<u64>, Vec<Board>) {
    let mut lines = input.lines();

    let draws = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    lines.next();

    let mut ret = Vec::new();
    let mut r = 0;
    let mut curr = [[(0, false); 5]; 5];
    lines
        .filter(|l| !l.is_empty())
        .map(|l| l.split(' ').filter_map(|s| s.parse().ok()))
        .for_each(|line| {
            for (i, x) in line.enumerate() {
                curr[r][i].0 = x;
            }
            r += 1;
            if r == 5 {
                r = 0;
                ret.push(curr);
            }
        });

    (draws, ret)
}

fn won(board: &Board) -> bool {
    board.iter().any(|r| r.iter().all(|x| x.1)) || (0..5).any(|i| board.iter().all(|r| r[i].1))
}

fn mark(x: u64, board: &mut Board) {
    for (v, b) in board.iter_mut().flatten() {
        if *v == x {
            *b = true;
        }
    }
}

fn score(x: u64, board: &Board) -> u64 {
    x * board
        .iter()
        .flatten()
        .filter(|v| !v.1)
        .map(|v| v.0)
        .sum::<u64>()
}

#[aoc(day4, part1)]
pub fn part1(input: &(Vec<u64>, Vec<Board>)) -> u64 {
    let (draws, boards) = input;
    let mut boards = boards.clone();

    for &draw in draws {
        for b in &mut boards {
            mark(draw, b);
        }

        if let Some(board) = boards.iter().find(|b| won(b)) {
            return score(draw, board);
        }
    }
    unreachable!()
}

#[aoc(day4, part2)]
pub fn part2(input: &(Vec<u64>, Vec<Board>)) -> u64 {
    let (draws, boards) = input;
    let mut boards = boards.clone();

    let mut last = 0;

    for &draw in draws {
        for b in &mut boards {
            mark(draw, b);
        }

        if let Some(board) = boards.iter().find(|b| won(b)) {
            last = score(draw, board);
            boards.retain(|b| !won(b));
        }
    }

    last
}
