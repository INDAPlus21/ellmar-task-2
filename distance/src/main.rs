use std::io;
use std::io::prelude::*;

fn main() {
    let input = io::stdin();

    let lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap())
        .collect::<Vec<String>>();

    solution(lines);

}

fn solution (input: Vec<String>) {
    let dimensions = input[0]
    .split_whitespace()
    .map(|string| string.parse::<u32>()
    .unwrap())
    .collect::<Vec<u32>>();
    
    let rows = dimensions[0];
    let columns = dimensions[1];

    for row in 0..rows {
        let horiz_dist = if row < rows - row {row + 1} else {rows - row};
        
        for column in 0..columns {
            let vert_dist = if column < columns - column {column + 1} else {columns - column};
            let min_dist = if horiz_dist <= vert_dist {horiz_dist} else {vert_dist};
        
            if min_dist < 10 {print!("{}", min_dist);} else {print!(".");}
        }
        print!("\n")
    } 
}