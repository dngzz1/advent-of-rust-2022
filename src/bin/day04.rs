use std::fs;

fn main() {
    let input = fs::read_to_string("src/data/day04.txt").unwrap();
    let mut count_1 = 0;
    let mut count_2 = 0;
    for line in input.lines() {
        let values = line
            .split(&['-', ','][..])
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        if values[0] <= values[2] && values[3] <= values[1]
            || values[2] <= values[0] && values[1] <= values[3]
        {
            count_1 += 1;
        }
        if values[2] <= values[1] && values[0] <= values[3]
            || values[0] <= values[3] && values[2] <= values[1]
        {
            count_2 += 1;
        }
    }
    println!("Solution 1: {}", count_1);
    println!("Solution 2: {}", count_2);
}
