use regex::Regex;
use std::fs;

enum CraneMethod {
    CrateMover9000,
    CrateMover9001,
}
fn main() {
    let (stacks, moves) = get_data("src/data/day05.txt");
    println!(
        "Part 1: {}",
        rearrange(stacks.clone(), &moves, CraneMethod::CrateMover9000)
    );
    println!(
        "Part 2: {}",
        rearrange(stacks.clone(), &moves, CraneMethod::CrateMover9001)
    );
}

fn rearrange(
    mut stacks: [Vec<char>; 9],
    moves: &[(usize, usize, usize)],
    crane_method: CraneMethod,
) -> String {
    for &(n, from_index, to_index) in moves {
        match crane_method {
            CraneMethod::CrateMover9000 => {
                rearrange_1(n, &mut stacks, from_index, to_index);
            }
            CraneMethod::CrateMover9001 => {
                rearrange_2(n, &mut stacks, from_index, to_index);
            }
        }
    }
    stacks.iter().map(|v| v.last().unwrap()).collect::<String>()
}

fn rearrange_1(n: usize, stacks: &mut [Vec<char>; 9], from_index: usize, to_index: usize) {
    for _ in 0..n {
        if let Some(ch) = stacks[from_index - 1].pop() {
            stacks[to_index - 1].push(ch);
        };
    }
}

fn rearrange_2(n: usize, stacks: &mut [Vec<char>; 9], from_index: usize, to_index: usize) {
    let s = stacks;
    let l = s[from_index - 1].len();
    let m = std::cmp::min(n, l);
    let carry = s[from_index - 1].split_off(l - m);
    s[to_index - 1].extend(carry);
}

fn get_data(path: &str) -> ([Vec<char>; 9], Vec<(usize, usize, usize)>) {
    let input = fs::read_to_string(path).unwrap();
    let (crates, moves) = {
        let v = input
            .split("\n\n")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        (v[0].to_owned(), v[1].to_owned())
    };
    let crates = get_crates(&crates);
    let moves = get_moves(&moves);
    (crates, moves)
}

fn get_crates(crates: &str) -> [Vec<char>; 9] {
    let mut result: [Vec<char>; 9] = Default::default();
    for line in crates.lines() {
        let mut index = 0;
        let mut c = line.chars();
        while let Some(ch) = c.next() {
            if index % 4 == 1 && !vec![' ', '[', ']', '\n'].contains(&ch) {
                let crate_index = (index - 1) / 4;
                result[crate_index].push(ch);
            }
            index += 1;
        }
    }
    for vec in &mut result {
        vec.pop();
        vec.reverse();
    }
    result
}
// 0 -> 1, 1 -> 5, 2-> 9

fn get_moves(moves: &str) -> Vec<(usize, usize, usize)> {
    let mut result = vec![];
    let re = Regex::new(r"^move (\d*) from (\d*) to (\d*)$").unwrap();
    for m in moves.lines() {
        let caps = re.captures_iter(m);
        for cap in caps {
            let values = (
                cap[1].parse::<usize>().unwrap(),
                cap[2].parse::<usize>().unwrap(),
                cap[3].parse::<usize>().unwrap(),
            );
            result.push(values);
        }
    }
    result
}
