use petgraph::algo::dijkstra;
use petgraph::prelude::DiGraphMap;

pub type Input = Grid<100>;

pub type Grid<const N: usize> = [[u16; N]; N];

#[aoc_generator(day15)]
pub fn generator(input: &str) -> Input {
    let mut ret: Input = [[0; 100]; 100];
    for (line, row) in input.lines().zip(&mut ret) {
        for (c, x) in line.chars().zip(row) {
            *x = (c as u8 - b'0') as u16;
        }
    }
    ret
}

fn make_graph<const N: usize>(input: &Grid<N>) -> DiGraphMap<(usize, usize), u16> {
    let mut ret = DiGraphMap::new();
    for (r, line) in input.iter().enumerate() {
        for (c, &x) in line.iter().enumerate() {
            if r != 0 {
                ret.add_edge((r - 1, c), (r, c), x);
            }
            if c != 0 {
                ret.add_edge((r, c - 1), (r, c), x);
            }
            if r != input.len() - 1 {
                ret.add_edge((r + 1, c), (r, c), x);
            }
            if c != input.len() - 1 {
                ret.add_edge((r, c + 1), (r, c), x);
            }
        }
    }
    ret
}

#[aoc(day15, part1)]
pub fn part1(input: &Input) -> u64 {
    let graph = make_graph(input);
    let max = (input.len() - 1, input[0].len() - 1);

    dijkstra(&graph, (0, 0), Some(max), |e| *e.2 as u64)
        .get(&max)
        .copied()
        .unwrap()
}

#[aoc(day15, part2)]
pub fn part2(input: &Input) -> u64 {
    let mut grid: Grid<500> = [[0; 500]; 500];
    for rx in 0..5 {
        for cx in 0..5 {
            for (old_row, new_row) in input.iter().zip(&mut grid[100 * rx..100 * (rx + 1)]) {
                for (&old_x, new_x) in old_row.iter().zip(&mut new_row[100 * cx..100 * (cx + 1)]) {
                    *new_x = old_x + (rx + cx) as u16;
                    if *new_x > 9 {
                        *new_x -= 9;
                    }
                }
            }
        }
    }

    let graph = make_graph(&grid);
    let max: (usize, usize) = (499, 499);

    dijkstra(&graph, (0, 0), Some(max), |e| *e.2 as u64)
        .get(&max)
        .copied()
        .unwrap()
}
