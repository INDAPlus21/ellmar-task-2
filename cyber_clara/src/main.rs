use std::io;
use std::io::prelude::*;

fn main() {
    let input = io::stdin();

    let lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap())
        .collect::<Vec<String>>();
    let answer = solution(lines);
    println!("{}", answer);
}

fn solution (lines: Vec<String>) -> usize {
    let len = lines[0]
        .parse::<u32>()
        .unwrap();
    
    let mut names = Vec::new();
    for idx in 1..len+1 {
        names.push((lines[idx as usize].clone(), lines[(idx + len) as usize].clone()));
    }
    names.sort();
    names.dedup();

    names.len()
}