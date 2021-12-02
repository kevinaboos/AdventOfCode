//! Solution for <https://adventofcode.com/2021/day/1>

use std::{error::Error, fs::File, io::{BufReader, BufRead}, usize};

pub fn part1() -> Result<usize, Box<dyn Error>> {
    let input = File::open("input/dec_01.txt")?;
    let reader = BufReader::new(input);
    let mut num_increases = 0;

    let mut prev_val = usize::MAX;
    for line in reader.lines() {
        let val: usize = line?.trim().parse()?;
        if val > prev_val {
            num_increases += 1;
        }
        prev_val = val;
    }

    Ok(num_increases)
}

pub fn part2() -> Result<usize, Box<dyn Error>> {
    let input = File::open("input/dec_01.txt")?;
    let reader = BufReader::new(input);
    let mut num_increases = 0;

    let mut prev_sum = usize::MAX;
    let all_lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();
    for window in all_lines.windows(3) {
        let sum = window.iter().fold(0, |acc, line| acc + line.trim().parse::<usize>().unwrap());
        if sum > prev_sum {
            num_increases += 1;
        }
        prev_sum = sum;
    }

    Ok(num_increases)
}