#[aoc(day3, part1)]
pub fn part1(input: &str) -> u64 {
    let line_len = input.find('\n').unwrap();
    let num_lines = input.lines().count();
    let needed = num_lines / 2 + num_lines % 2;

    let mut gamma = String::new();
    for i in 0..line_len {
        let num_ones = input
            .lines()
            .filter(|l| l.chars().nth(i).unwrap() == '1')
            .count();
        if num_ones >= needed {
            gamma.push('1');
        } else {
            gamma.push('0');
        }
    }

    let gamma = u64::from_str_radix(&gamma, 2).unwrap();
    let epsilon = !gamma & ((1 << line_len) - 1);

    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u64 {
    let mut oxy: Vec<_> = input.lines().collect();
    let mut co2: Vec<_> = input.lines().collect();

    let mut i = 0;
    while oxy.len() > 1 {
        let most_common = if oxy
            .iter()
            .filter(|x| x.chars().nth(i).unwrap() == '1')
            .count()
            >= oxy.len() / 2 + oxy.len() % 2
        {
            '1'
        } else {
            '0'
        };
        oxy = oxy
            .into_iter()
            .filter(|x| x.chars().nth(i).unwrap() == most_common)
            .collect();
        i += 1;
    }

    i = 0;
    while co2.len() > 1 {
        let least_common = if co2
            .iter()
            .filter(|x| x.chars().nth(i).unwrap() == '1')
            .count()
            < co2.len() / 2 + co2.len() % 2
        {
            '1'
        } else {
            '0'
        };
        co2 = co2
            .into_iter()
            .filter(|x| x.chars().nth(i).unwrap() == least_common)
            .collect();
        i += 1;
    }

    u64::from_str_radix(oxy[0], 2).unwrap() * u64::from_str_radix(co2[0], 2).unwrap()
}
