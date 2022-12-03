use std::fs;

fn main() {
    let input = fs::read_to_string("src/data/day01.txt").unwrap();
    let mut elves = vec![];
    for bag in input.split("\n\n") {
        let mut calories = 0;
        for calorie in bag.lines() {
            let x = calorie.parse::<i32>().unwrap();
            calories += x;
        }
        elves.push(calories);
    }
    println!("Max: {}", elves.iter().max().unwrap());
    elves.sort();
    println!("Sum of top 3: {}", elves.iter().rev().take(3).sum::<i32>());
}
