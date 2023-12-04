use crate::AdventErr::InputParse;
use crate::{parser, utils, AdventResult};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;

//noinspection DuplicatedCode
pub fn run(mut input_file: File) -> AdventResult<()> {
    let scratch_cards = parser::as_vec_by_line(&mut input_file, line_parser)?;

    // Part 1
    utils::part_header(1);
    part_1(&scratch_cards)?;

    // Part 2
    utils::part_header(2);
    part_2(&scratch_cards)?;

    Ok(())
}

fn part_1(scratch_cards: &[ScratchCard]) -> AdventResult<()> {
    let score_sum: u64 = scratch_cards.iter().map(|card| card.score()).sum();

    println!("Sum of card scores: {score_sum}");

    Ok(())
}

fn part_2(card_spec: &[ScratchCard]) -> AdventResult<()> {
    let mut card_count = vec![1_u64; card_spec.len()];

    for (card_id, card) in card_spec.iter().enumerate() {
        let current_count = card_count[card_id];
        let match_count = card.matches() as usize;
        for offset in 1..=match_count {
            card_count[card_id + offset] += current_count;
        }
    }

    let total_cards: u64 = card_count.iter().sum();

    println!("Total number of cards: {total_cards}");

    Ok(())
}

#[derive(Debug)]
struct ScratchCard {
    winning_nums: HashSet<u32>,
    your_nums: HashSet<u32>,
}

impl ScratchCard {
    fn matches(&self) -> u32 {
        self.winning_nums.intersection(&self.your_nums).count() as u32
    }

    fn score(&self) -> u64 {
        match self.matches() {
            0 => 0,
            n => 2_u64.pow(n - 1),
        }
    }
}

lazy_static! {
    static ref LINE_REGEX: Regex =
        Regex::new(r"Card\s+(?<id>\d+):(?<winning>.+?)\|(?<yours>.+)").unwrap();
    static ref NUMBER_REGEX: Regex = Regex::new(r"\d+").unwrap();
}

fn line_parser(line: &str) -> AdventResult<ScratchCard> {
    let Some(caps) = LINE_REGEX.captures(line) else {
        return Err(InputParse(format!("Malformed input - line:\n{line}")));
    };

    let winning_nums = NUMBER_REGEX
        .find_iter(&caps["winning"])
        .map(|m| {
            m.as_str()
                .parse()
                .map_err(|_| InputParse(format!("Failed to parse winning number - line:\n{line}")))
        })
        .collect::<AdventResult<_>>()?;

    let your_nums = NUMBER_REGEX
        .find_iter(&caps["yours"])
        .map(|m| {
            m.as_str()
                .parse()
                .map_err(|_| InputParse(format!("Failed to parse your number - line:\n{line}")))
        })
        .collect::<AdventResult<_>>()?;

    Ok(ScratchCard {
        winning_nums,
        your_nums,
    })
}
