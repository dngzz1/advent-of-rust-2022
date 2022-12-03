use std::{collections::HashSet, fs};

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = fs::read_to_string("src/data/day03.txt").unwrap();
    let mut sum_of_priorities = 0;
    for line in input.lines() {
        let n = line.chars().count();
        assert!(n % 2 == 0);
        let mut c = line.chars();
        let compartment_1 = {
            let mut v = vec![];
            for _ in 0..(n / 2) {
                let letter = c.next().unwrap();
                v.push(letter);
            }
            v.into_iter().collect::<HashSet<_>>()
        };
        let compartment_2 = {
            let mut v = vec![];
            for _ in 0..(n / 2) {
                let letter = c.next().unwrap();
                v.push(letter);
            }
            v.into_iter().collect::<HashSet<_>>()
        };
        let intersection = compartment_1
            .intersection(&compartment_2)
            .collect::<Vec<_>>()[0];
        sum_of_priorities += priority(intersection);
    }
    println!("Part 1, sum of priorities: {}", sum_of_priorities);
}

fn part_2() {
    let input = fs::read_to_string("src/data/day03.txt").unwrap();
    let mut sum_of_priorities = 0;
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let line0 = string_to_set(line);
        let line1 = string_to_set(lines.next().unwrap());
        let line2 = string_to_set(lines.next().unwrap());
        let intersection = line0
            .intersection(&line1)
            .map(|c| *c)
            .collect::<HashSet<_>>();
        let intersection2 = intersection.intersection(&line2).collect::<Vec<&char>>();
        sum_of_priorities += priority(intersection2[0]);
    }
    println!("Part 2, sum of priorities: {}", sum_of_priorities);
}

fn string_to_set(s: &str) -> HashSet<char> {
    s.chars().collect::<HashSet<_>>()
}

fn priority(c: &char) -> usize {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();
    alphabet.iter().position(|r| r == c).unwrap() + 1
}
