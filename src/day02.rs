#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let mut x = 0;
    let mut y = 0;
    input.lines().for_each(|line| {
        let (dir, dx) = line.split_once(' ').unwrap();
        let dx: i32 = dx.parse().unwrap();
        match dir {
            "forward" => {
                x += dx;
            }
            "down" => {
                y += dx;
            }
            "up" => {
                y -= dx;
            }
            _ => unreachable!(),
        }
    });
    x * y
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    input.lines().for_each(|line| {
        let (dir, dx) = line.split_once(' ').unwrap();
        let dx: i32 = dx.parse().unwrap();
        match dir {
            "forward" => {
                x += dx;
                y += aim * dx;
            }
            "down" => {
                aim += dx;
            }
            "up" => {
                aim -= dx;
            }
            _ => unreachable!(),
        }
    });
    x * y
}
