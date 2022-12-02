use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("./input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut total_score = 0;
    for line in lines {
        total_score += match line.unwrap().as_str() {
            "A X" => 3 + 1,
            "A Y" => 6 + 2,
            "A Z" => 0 + 3,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 6 + 1,
            "C Y" => 0 + 2,
            "C Z" => 3 + 3,
            _ => 0,
        }
    }

    println!("{}", total_score);
}

fn part2() {
    let file = File::open("./input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut total_score = 0;
    for line in lines {
        total_score += match line.unwrap().as_str() {
            "A X" => 0 + 3,
            "A Y" => 3 + 1,
            "A Z" => 6 + 2,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 0 + 2,
            "C Y" => 3 + 3,
            "C Z" => 6 + 1,
            _ => 0,
        }
    }

    println!("{}", total_score);
}
