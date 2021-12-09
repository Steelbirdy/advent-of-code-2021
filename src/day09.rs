pub type Input = [[u8; COLS]; ROWS];

const ROWS: usize = 100;
const COLS: usize = 100;

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Input {
    let mut arr = [[0; COLS]; ROWS];

    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.bytes().enumerate() {
            arr[r][c] = ch - b'0';
        }
    }

    arr
}

#[aoc(day9, part1)]
pub fn part1(input: &Input) -> u16 {
    let mut ret = 0;

    // corners
    let mut x = input[0][0];
    if x < input[0][1] && x < input[1][0] {
        ret += x as u16 + 1;
    }

    x = input[ROWS - 1][0];
    if x < input[ROWS - 1][1] && x < input[ROWS - 2][0] {
        ret += x as u16 + 1;
    }

    x = input[0][COLS - 1];
    if x < input[0][COLS - 2] && x < input[1][COLS - 1] {
        ret += x as u16 + 1;
    }

    x = input[ROWS - 1][COLS - 1];
    if x < input[ROWS - 1][COLS - 2] && x < input[ROWS - 2][COLS - 1] {
        ret += x as u16 + 1;
    }

    // top row
    let (r1, r2) = (&input[0], &input[1]);
    for (xyz, &x2) in r1.windows(3).zip(&r2[1..COLS - 1]) {
        let (y, x, z) = match *xyz {
            [y, x, z] => (y, x, z),
            _ => unreachable!(),
        };

        if x < y && x < x2 && x < z {
            ret += x as u16 + 1;
        }
    }

    // bottom row
    let (r1, r2) = (&input[ROWS - 1], &input[ROWS - 2]);
    for (xyz, &x2) in r1.windows(3).zip(&r2[1..COLS - 1]) {
        let (y, x, z) = match *xyz {
            [y, x, z] => (y, x, z),
            _ => unreachable!(),
        };

        if x < y && x < x2 && x < z {
            ret += x as u16 + 1;
        }
    }

    for rows in input.windows(3) {
        let (r1, r2, r3) = match rows {
            [r1, r2, r3] => (r1, r2, r3),
            _ => unreachable!(),
        };

        // ends
        x = r2[0];
        if x < r1[0] && x < r2[1] && x < r3[0] {
            ret += x as u16 + 1;
        }

        x = r2[COLS - 1];
        if x < r1[COLS - 1] && x < r2[COLS - 2] && x < r3[COLS - 1] {
            ret += x as u16 + 1;
        }

        for xs in r2.windows(3).zip(&r1[1..COLS - 1]).zip(&r3[1..COLS - 1]) {
            let ((xyz, &x1), &x2) = xs;
            let (y, x, z) = match *xyz {
                [y, x, z] => (y, x, z),
                _ => unreachable!(),
            };

            if x < x1 && x < x2 && x < y && x < z {
                ret += x as u16 + 1;
            }
        }
    }

    ret
}

#[inline]
fn basin_size(r: usize, c: usize, input: &mut Input) -> usize {
    if input[r][c] == u8::MAX {
        return 0;
    }
    input[r][c] = u8::MAX;

    let mut ret = 1;
    if r != 0 && input[r - 1][c] < 9 {
        ret += basin_size(r - 1, c, input);
    }
    if c != 0 && input[r][c - 1] < 9 {
        ret += basin_size(r, c - 1, input);
    }
    if r != ROWS - 1 && input[r + 1][c] < 9 {
        ret += basin_size(r + 1, c, input);
    }
    if c != COLS - 1 && input[r][c + 1] < 9 {
        ret += basin_size(r, c + 1, input);
    }

    ret
}

#[aoc(day9, part2)]
pub fn part2(input: &Input) -> usize {
    let mut marked_arr = *input;

    let mut highest = 0;
    let mut second = 0;
    let mut third = 0;

    macro_rules! adjust {
        ($x:expr, $y:expr) => {{
            let basin_size = basin_size($x, $y, &mut marked_arr);
            if basin_size > highest {
                third = second;
                second = highest;
                highest = basin_size;
            } else if basin_size > second {
                third = second;
                second = basin_size;
            } else if basin_size > third {
                third = basin_size;
            }
        }};
    }

    // corners
    let mut x = input[0][0];
    if x < input[0][1] && x < input[1][0] {
        adjust!(0, 0);
    }

    x = input[ROWS - 1][0];
    if x < input[ROWS - 1][1] && x < input[ROWS - 2][0] {
        adjust!(ROWS - 1, 0);
    }

    x = input[0][COLS - 1];
    if x < input[0][COLS - 2] && x < input[1][COLS - 1] {
        adjust!(0, COLS - 1);
    }

    x = input[ROWS - 1][COLS - 1];
    if x < input[ROWS - 1][COLS - 2] && x < input[ROWS - 2][COLS - 1] {
        adjust!(ROWS - 1, COLS - 1);
    }

    let mut c = 1;

    // top row
    let (r1, r2) = (&input[0], &input[1]);
    for (xyz, &x2) in r1.windows(3).zip(&r2[1..COLS - 1]) {
        let (y, x, z) = match *xyz {
            [y, x, z] => (y, x, z),
            _ => unreachable!(),
        };

        if x < y && x < x2 && x < z {
            adjust!(0, c);
        }

        c += 1;
    }

    // bottom row
    c = 1;
    let (r1, r2) = (&input[ROWS - 1], &input[ROWS - 2]);
    for (xyz, &x2) in r1.windows(3).zip(&r2[1..COLS - 1]) {
        let (y, x, z) = match *xyz {
            [y, x, z] => (y, x, z),
            _ => unreachable!(),
        };

        if x < y && x < x2 && x < z {
            adjust!(ROWS - 1, c);
            c += 1;
        }
    }

    let mut r = 1;
    for rows in input.windows(3) {
        let (r1, r2, r3) = match rows {
            [r1, r2, r3] => (r1, r2, r3),
            _ => unreachable!(),
        };

        // ends
        x = r2[0];
        if x < r1[0] && x < r2[1] && x < r3[0] {
            adjust!(r, 0);
        }

        x = r2[COLS - 1];
        if x < r1[COLS - 1] && x < r2[COLS - 2] && x < r3[COLS - 1] {
            adjust!(r, COLS - 1);
        }

        c = 1;
        for xs in r2.windows(3).zip(&r1[1..COLS - 1]).zip(&r3[1..COLS - 1]) {
            let ((xyz, &x1), &x2) = xs;
            let (y, x, z) = match *xyz {
                [y, x, z] => (y, x, z),
                _ => unreachable!(),
            };

            if x < x1 && x < x2 && x < y && x < z {
                adjust!(r, c);
            }

            c += 1;
        }

        r += 1;
    }

    highest * second * third
}
