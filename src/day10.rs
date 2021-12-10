const STACK_LEN: usize = 50;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> u32 {
    let mut stack = [0_u8; STACK_LEN];
    let mut sp;

    let mut ret = 0;

    for line in input.lines() {
        sp = 0;
        for ch in line.bytes() {
            if ch == b'(' || ch == b'[' || ch == b'{' || ch == b'<' {
                stack[sp] = ch;
                sp += 1;
            } else {
                sp -= 1;
                // Significantly faster than popping a u8 off an actual stack
                // The difference between opening and closing characters is always 3 or less
                if ch - stack[sp] > 3 {
                    ret += match ch {
                        b')' => 3,
                        b']' => 57,
                        b'}' => 1197,
                        b'>' => 25137,
                        _ => unreachable!(),
                    };
                }
            }
        }
    }

    ret
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> u64 {
    let mut stack = [0_u8; STACK_LEN];
    let mut sp;

    let mut scores = Vec::with_capacity(50);

    'outer: for line in input.lines() {
        sp = 0;
        for ch in line.bytes() {
            if ch == b'(' || ch == b'[' || ch == b'{' || ch == b'<' {
                stack[sp] = ch;
                sp += 1;
            } else {
                sp -= 1;
                if ch - stack[sp] > 3 {
                    // Hooray for loop labels!
                    continue 'outer;
                }
            }
        }

        let score = stack[..sp].iter().rev().fold(0, |score, ch| {
            5 * score
                + match ch {
                    b'(' => 1,
                    b'[' => 2,
                    b'{' => 3,
                    b'<' => 4,
                    _ => unreachable!(),
                }
        });
        scores.push(score);
    }

    scores.sort_unstable();
    scores[scores.len() / 2]
}
