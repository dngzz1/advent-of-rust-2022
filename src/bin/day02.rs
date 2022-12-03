use std::fs;

fn main() {
    let input = fs::read_to_string("src/data/day02.txt").unwrap();
    let mut total_score_1 = 0;
    let mut total_score_2 = 0;
    for line in input.lines() {
        let mut c = line.chars();
        let a = c.next().unwrap();
        c.next();
        let b = c.next().unwrap();
        let win_score_1 = match (a, b) {
            ('A', 'X') => 3 + 1,
            ('A', 'Y') => 6 + 2,
            ('A', 'Z') => 0 + 3,
            ('B', 'X') => 0 + 1,
            ('B', 'Y') => 3 + 2,
            ('B', 'Z') => 6 + 3,
            ('C', 'X') => 6 + 1,
            ('C', 'Y') => 0 + 2,
            ('C', 'Z') => 3 + 3,
            _ => panic!(),
        };
        total_score_1 += win_score_1;
        // A = Rock = 1, B = Paper = 2, C = Scissors = 3
        let win_score_2 = match (a, b) {
            ('A', 'X') => 0 + 3,
            ('A', 'Y') => 3 + 1,
            ('A', 'Z') => 6 + 2,
            ('B', 'X') => 0 + 1,
            ('B', 'Y') => 3 + 2,
            ('B', 'Z') => 6 + 3,
            ('C', 'X') => 0 + 2,
            ('C', 'Y') => 3 + 3,
            ('C', 'Z') => 6 + 1,
            _ => panic!(),
        };
        total_score_2 += win_score_2;
    }
    println!("Part 1 total score: {}", total_score_1);
    println!("Part 2 total score: {}", total_score_2);
}
