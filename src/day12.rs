use std::collections::{HashMap, HashSet};

type Neighbors = HashMap<String, Vec<String>>;

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Neighbors {
    let mut neighbors = Neighbors::new();
    for line in input.lines() {
        let (start, end) = line.split_once('-').unwrap();
        neighbors
            .entry(start.to_string())
            .or_default()
            .push(end.to_string());
        neighbors
            .entry(end.to_string())
            .or_default()
            .push(start.to_string());
    }
    neighbors
}

// Pretty standard DFS, only changes are to disallow visiting lowercase nodes twice/three times
fn search<'a>(
    start: &str,
    seen: &mut HashSet<&'a str>,
    neighbors: &'a Neighbors,
    allow_twice: bool,
) -> u32 {
    if start == "end" {
        return 1;
    }

    let mut ret = 0;
    for n in neighbors.get(start).unwrap() {
        let n = n as &str;
        if !seen.contains(n) {
            if !n.contains(|c: char| c.is_ascii_uppercase()) {
                seen.insert(n);
                ret += search(n, seen, neighbors, allow_twice);
                seen.remove(n);
            } else {
                ret += search(n, seen, neighbors, allow_twice);
            }
        } else if allow_twice && n != "start" {
            ret += search(n, seen, neighbors, false);
        }
    }
    ret
}

#[aoc(day12, part1)]
pub fn part1(input: &Neighbors) -> u32 {
    search("start", &mut HashSet::from(["start"]), input, false)
}

#[aoc(day12, part2)]
pub fn part2(input: &Neighbors) -> u32 {
    search("start", &mut HashSet::from(["start"]), input, true)
}
