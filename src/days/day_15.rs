use crate::AdventErr::InputParse;
use crate::{parser, utils, AdventResult};
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::mem;
use std::mem::MaybeUninit;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let init_sequence =
        parser::as_vec_by_block(&mut input_file, ",", |block| Ok(block.to_string()))?;

    // Part 1
    utils::part_header(1);
    part_1(&init_sequence);

    // Part 2
    utils::part_header(2);
    part_2(&init_sequence)?;

    Ok(())
}

fn part_1(init_sequence: &[String]) {
    let hash_sum: usize = init_sequence.iter().map(|s| aoc_hash(s)).sum();

    println!("Sum of initialization HASH: {hash_sum}");
}

fn part_2(init_sequence: &[String]) -> AdventResult<()> {
    let mut boxes = {
        let mut boxes: [MaybeUninit<LightBox>; 256] =
            unsafe { MaybeUninit::uninit().assume_init() };

        for elem in &mut boxes {
            elem.write(LightBox::new());
        }

        unsafe { mem::transmute::<_, [LightBox; 256]>(boxes) }
    };

    for instruction in init_sequence {
        execute_instruction(instruction, &mut boxes)?;
    }

    let focusing_power: u64 = boxes
        .iter()
        .enumerate()
        .map(|(box_id, light_box)| {
            light_box
                .lenses
                .iter()
                .enumerate()
                .map(|(lens_slot, lens)| {
                    (box_id as u64 + 1) * (lens_slot as u64 + 1) * lens.focal_length as u64
                })
                .sum::<u64>()
        })
        .sum();

    println!("Focusing power: {focusing_power}");

    Ok(())
}

fn execute_instruction(instruction: &str, boxes: &mut [LightBox]) -> AdventResult<()> {
    let Some(caps) = INSTRUCTION_REGEX.captures(instruction) else {
        return Err(InputParse(format!(
            "Failed to match instruction '{instruction}'"
        )));
    };

    let label = caps.name("label").unwrap().as_str();
    let box_id = aoc_hash(label);
    let operation = caps.name("op").unwrap().as_str();

    match operation {
        "-" => {
            boxes[box_id].remove(label);
            Ok(())
        }

        "=" => {
            let Some(focal_length) = caps.name("focal_length") else {
                return Err(InputParse(String::from(
                    "Missing focal length on insert operation",
                )));
            };
            let focal_length = focal_length.as_str();
            let focal_length: u8 = focal_length.parse().map_err(|_| {
                InputParse(format!("Failed to parse focal length '{focal_length}'"))
            })?;

            boxes[box_id].set(label, focal_length);
            Ok(())
        }

        _ => Err(InputParse(format!("Unrecognized operation {operation}"))),
    }
}

fn aoc_hash(s: &str) -> usize {
    s.chars().fold(0, |mut acc, c| {
        acc += c as usize;
        acc *= 17;
        acc % 256
    })
}

lazy_static! {
    static ref INSTRUCTION_REGEX: Regex =
        Regex::new(r"(?<label>[A-Za-z]+)(?<op>[\-=])(?<focal_length>\d)?").unwrap();
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: u8,
}

#[derive(Debug)]
struct LightBox {
    lenses: Vec<Lens>,
}

impl LightBox {
    fn new() -> Self {
        Self { lenses: vec![] }
    }

    fn remove(&mut self, label: &str) {
        if let Some(pos) = self.lenses.iter().position(|lens| lens.label == label) {
            self.lenses.remove(pos);
        }
    }

    fn set(&mut self, label: &str, focal_length: u8) {
        if let Some(lens) = self.lenses.iter_mut().find(|lens| lens.label == label) {
            lens.focal_length = focal_length;
        } else {
            self.lenses.push(Lens {
                label: label.to_string(),
                focal_length,
            })
        }
    }
}
