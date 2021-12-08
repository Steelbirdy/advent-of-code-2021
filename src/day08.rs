use std::collections::{HashMap, HashSet};

pub type Input = (Vec<String>, Vec<String>);

pub type Output = usize;

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|l| {
            let (l, r) = l.split_once(" | ").unwrap();
            (
                l.split(' ').map(|s| s.trim().to_string()).collect(),
                r.split(' ').map(|s| s.trim().to_string()).collect(),
            )
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn part1(input: &[Input]) -> Output {
    input
        .iter()
        .flat_map(|line| &line.1)
        .filter(|s| {
            let len = s.len();
            len == 2 || len == 3 || len == 4 || len == 7
        })
        .count()
}

#[derive(Default)]
struct Encodings {
    map: HashMap<char, HashSet<char>>,
}

impl Encodings {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, k: char, v: HashSet<char>) {
        self.map.entry(k).or_insert(v);
    }

    fn diff_count(&self, k: char, b: &HashSet<char>) -> usize {
        self.map
            .get(&k)
            .expect("key not found in encodings")
            .difference(b)
            .count()
    }

    fn contains(&self, k: char) -> bool {
        self.map.contains_key(&k)
    }

    fn get_diff(&self, a: char, b: char) -> impl Iterator<Item = &char> {
        self.map
            .get(&a)
            .unwrap()
            .difference(self.map.get(&b).unwrap())
    }
}

/// Determines the correct string encoding for each digit '0'-'9'
///
/// We refer to the segments as follows:
///```markdown
///  aaaa
/// b    c
/// b    c
///  dddd
/// e    f
/// e    f
///  gggg
///```
/// Currently known: None
///
/// # Stage 1
/// As shown in Part 1, '1' is the only number with 2 segments, '7' the only one
/// with 3 segments, '4' the only with 4 segments, and '8' the only with 7 segments.
/// When we find a string of one of those lengths, we know for sure which one it is.
///
/// Currently known: {'1', '4', '7', '8'}
///
/// # Stage 2
/// '6', '9', and '0' all have 6 segments.
/// * '6' is the only number that doesn't share both segment *c* and segment *f* with '1'.
/// So we take the set difference of the string already stored for '1' and take the set
/// difference with the current string character-wise. If the difference is not empty,
/// we know we are looking at 6.
/// * The same is true for '4' and '0'.
/// * Otherwise, a string with 6 segments must be a '9'.
///
/// Currently known: {'0', '1', '4', '6', '7', '8', '9'}
///
/// # Stage 3
/// The only numbers left ('2', '3', and '5') all have 5 segments.
/// * '3' is the only number with 5 segments that shares both *c* and *f* with '1'.
/// * Out of '2' and '5', only '2' uses segment 'e'.
/// * Otherwise the string must represent '5'.
fn get_digits(signals: &[String]) -> HashMap<char, String> {
    let mut ret = Encodings::new();

    fn char_set(s: &str) -> HashSet<char> {
        s.chars().collect()
    }

    for signal in signals {
        let ch = match signal.len() {
            2 => '1',
            3 => '7',
            4 => '4',
            7 => '8',
            _ => continue,
        };
        ret.insert(ch, char_set(signal));
    }

    for signal in signals.iter().filter(|c| c.len() == 6) {
        let chars = char_set(signal);

        if !ret.contains('6') && ret.diff_count('1', &chars) != 0 {
            ret.insert('6', chars);
        } else if !ret.contains('0') && ret.diff_count('4', &chars) != 0 {
            ret.insert('0', chars);
        } else if !ret.contains('9') {
            ret.insert('9', chars);
        }
    }

    let segment_e = ret.get_diff('6', '9').next().copied().unwrap();
    for signal in signals.iter().filter(|c| c.len() == 5) {
        let chars = char_set(signal);

        if ret.diff_count('1', &chars) == 0 {
            ret.insert('3', chars);
        } else if signal.contains(segment_e) {
            ret.insert('2', chars);
        } else {
            ret.insert('5', chars);
        }
    }

    ret.map
        .into_iter()
        .map(|(c, ch)| (c, ch.into_iter().collect()))
        .collect()
}

fn sort_str(s: &str) -> String {
    let mut ch = s.chars().collect::<Vec<_>>();
    ch.sort_unstable();
    ch.into_iter().collect()
}

#[aoc(day8, part2)]
pub fn part2(input: &[Input]) -> Output {
    input
        .iter()
        .map(|(signals, out_signals)| {
            let digits = get_digits(signals);
            let digits_rev = digits
                .into_iter()
                .map(|(k, v)| (sort_str(&v), k))
                .collect::<HashMap<_, _>>();

            out_signals
                .iter()
                .map(|s| {
                    let s = sort_str(s);
                    digits_rev.get(&s).copied().unwrap()
                })
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}
