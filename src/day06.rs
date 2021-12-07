use parse_display::{Display, FromStr};

pub type Input = u8;

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<Input> {
    input.lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &[Input]) -> usize {
    let start = input.len();
    let mut days = Vec::new();

    let mut new_input = input.to_vec();
    for day in 1..=9_u8 {
        let mut new_fish = 0;
        new_input.iter_mut()
            .for_each(|x| {
                if *x == 0 {
                    new_fish += 1;
                    *x = 6;
                } else {
                    *x -= 1;
                }
            });
        if day <= 4 {
            new_input.extend(std::iter::repeat(8).take(new_fish));
        }
        days.push(new_fish);
    }
    for x in 10..=80 {
        days.push(days[x - 10] + days[x - 8]);
    }
    days.into_iter().sum::<usize>() + start
}

#[aoc(day6, part2)]
pub fn part2(input: &[Input]) -> usize {
    let start = input.len();
    let mut days = Vec::new();

    let mut new_input = input.to_vec();
    for day in 1..=9_u8 {
        let mut new_fish = 0;
        new_input.iter_mut()
            .for_each(|x| {
                if *x == 0 {
                    new_fish += 1;
                    *x = 6;
                } else {
                    *x -= 1;
                }
            });
        if day <= 4 {
            new_input.extend(std::iter::repeat(8).take(new_fish));
        }
        days.push(new_fish);
    }
    for x in 10..=256 {
        days.push(days[x - 10] + days[x - 8]);
    }
    days.into_iter().sum::<usize>() + start
}
