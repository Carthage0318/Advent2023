use crate::{parser, utils, AdventResult};
use std::fs::File;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let init_sequence =
        parser::as_vec_by_block(&mut input_file, ",", |block| Ok(block.to_string()))?;

    // Part 1
    utils::part_header(1);
    part_1(&init_sequence);

    Ok(())
}

fn part_1(init_sequence: &[String]) {
    let hash_sum: u64 = init_sequence.iter().map(|s| aoc_hash(s)).sum();

    println!("Sum of initialization HASH: {hash_sum}");
}

fn aoc_hash(s: &str) -> u64 {
    s.chars().fold(0, |mut acc, c| {
        acc += c as u64;
        acc *= 17;
        acc % 256
    })
}
