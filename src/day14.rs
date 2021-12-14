use itertools::Itertools;
use std::collections::HashMap;

pub type Input = (String, Vec<((char, char), char)>);

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Input {
    let mut lines = input.lines();
    let template = lines.next().unwrap().to_string();
    lines.next();

    let mut rules = Vec::new();
    for line in lines {
        let (a, b) = line.split_once(" -> ").unwrap();
        let mut a = a.chars();
        rules.push((
            (a.next().unwrap(), a.next().unwrap()),
            b.chars().next().unwrap(),
        ));
    }

    (template, rules)
}

fn step(template: &mut HashMap<(char, char), usize>, rules: &HashMap<(char, char), char>) {
    let mut new_template: HashMap<(char, char), usize> = HashMap::new();
    for (k, &v) in template.iter() {
        let c = rules.get(k).copied().unwrap();
        *new_template.entry((k.0, c)).or_default() += v;
        *new_template.entry((c, k.1)).or_default() += v;
    }

    *template = new_template;
}

fn run(input: &Input, steps: usize) -> usize {
    let (template_str, rules) = input;
    let rules = rules.iter().cloned().collect::<HashMap<_, _>>();

    let mut template = HashMap::new();
    for (a, b) in template_str.chars().tuple_windows::<(char, char)>() {
        *template.entry((a, b)).or_default() += 1;
    }

    for _ in 0..steps {
        step(&mut template, &rules);
    }

    let counts =
        template
            .into_iter()
            .fold(HashMap::<char, usize>::new(), |mut map, ((a, _), c)| {
                *map.entry(a).or_default() += c;
                map
            });

    counts.values().copied().max().unwrap() - counts.values().copied().min().unwrap() + 1
    // dammit
}

#[aoc(day14, part1)]
pub fn part1(input: &Input) -> usize {
    run(input, 10)
}

#[aoc(day14, part2)]
pub fn part2(input: &Input) -> usize {
    run(input, 40)
}
