//! Solution for <https://adventofcode.com/2021/day/2>

use std::{error::Error, fs::File, io::{BufReader, BufRead}, usize};

pub fn part1() -> Result<usize, Box<dyn Error>> {
    let input = File::open("input/dec_02.txt")?;
    let reader = BufReader::new(input);

    let (mut pos, mut depth) = (0, 0);

    for line in reader.lines() {
        let line = line?;
        let mut tokens = line.split_whitespace();
        let (cmd, x) = (
            tokens.next().unwrap(),
            tokens.next().unwrap().trim().parse::<usize>().unwrap(),
        );
        match cmd {
            "forward" => pos += x,
            "down" => depth += x,
            "up" => depth -= x,
            _other => return Err(format!("Invalid input line {:?}", line).into()),
        };
    }

    Ok(pos * depth)
}


pub fn part2() -> Result<usize, Box<dyn Error>> {
    let input = File::open("input/dec_02.txt")?;
    let reader = BufReader::new(input);

    let (mut pos, mut depth, mut aim) = (0, 0, 0);

    for line in reader.lines() {
        let line = line?;
        let mut tokens = line.split_whitespace();
        let (cmd, x) = (
            tokens.next().unwrap(),
            tokens.next().unwrap().trim().parse::<usize>().unwrap(),
        );
        match cmd {
            "forward" => {
                pos += x;
                depth += aim * x;
            }
            "down" => {
                aim += x;
            }
            "up" => {
                aim -= x;
            }
            _other => return Err(format!("Invalid input line {:?}", line).into()),
        };
    }

    Ok(pos * depth)
}
