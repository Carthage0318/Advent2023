use crate::AdventErr::InputParse;
use crate::{parser, utils, AdventErr, AdventResult};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let (instructions, nodes, map_spec) = parse_input(&mut input_file)?;

    // Part 1
    utils::part_header(1);
    part_1(&instructions, &nodes, map_spec);

    Ok(())
}

fn part_1(instructions: &[Instruction], nodes: &[Node], map_spec: MapSpec) {
    let steps = steps_to_walk(instructions, nodes, map_spec);

    println!("Steps required: {steps}");
}

fn steps_to_walk(instructions: &[Instruction], nodes: &[Node], map_spec: MapSpec) -> u64 {
    let mut count: u64 = 0;
    let mut current = map_spec.start;
    while current != map_spec.end {
        let instruction_index = count as usize % instructions.len();
        let node = &nodes[current];
        current = match instructions[instruction_index] {
            Instruction::Left => node.left,
            Instruction::Right => node.right,
        };
        count += 1;
    }

    count
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Instruction {
    Left,
    Right,
}

impl TryFrom<char> for Instruction {
    type Error = AdventErr;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            _ => Err(InputParse(format!(
                "Invalid character for instruction '{value}'"
            ))),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Node {
    left: usize,
    right: usize,
}

impl Node {
    fn empty() -> Self {
        Self { left: 0, right: 0 }
    }
}

#[derive(Debug, Copy, Clone)]
struct MapSpec {
    start: usize,
    end: usize,
}

lazy_static! {
    static ref LINE_REGEX: Regex =
        Regex::new(r"(?<this>[A-Z]+)\s+=\s+\((?<left>[A-Z]+),\s+(?<right>[A-Z]+)\)").unwrap();
}

fn parse_input<'a>(input_file: &mut File) -> AdventResult<(Vec<Instruction>, Vec<Node>, MapSpec)> {
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    let Some((instructions, nodes_str)) = input.split_once("\n\n") else {
        return Err(InputParse(String::from("Failed to split input")));
    };

    let instructions = parser::as_vec_by_char(instructions, Instruction::try_from)?;

    let mut nodes = Vec::new();
    let mut node_names_to_index = HashMap::new();

    let mut start = None;
    let mut end = None;

    for line in nodes_str.lines() {
        let Some(caps) = LINE_REGEX.captures(line) else {
            return Err(InputParse(format!("Unable to parse line:\n{line}")));
        };

        let this_name = caps.name("this").unwrap().as_str();
        let left_name = caps.name("left").unwrap().as_str();
        let right_name = caps.name("right").unwrap().as_str();

        let this = get_node_index(this_name, &mut nodes, &mut node_names_to_index);
        let left = get_node_index(left_name, &mut nodes, &mut node_names_to_index);
        let right = get_node_index(right_name, &mut nodes, &mut node_names_to_index);

        nodes[this] = Node { left, right };

        if start.is_none() && this_name == "AAA" {
            start = Some(this);
        } else if end.is_none() && this_name == "ZZZ" {
            end = Some(this);
        }
    }

    Ok((
        instructions,
        nodes,
        MapSpec {
            start: start.ok_or_else(|| InputParse(String::from("Failed to find start node")))?,
            end: end.ok_or_else(|| InputParse(String::from("Failed to find end node")))?,
        },
    ))
}

fn get_node_index<'a: 'b, 'b>(
    name: &'a str,
    nodes: &mut Vec<Node>,
    node_names_to_index: &mut HashMap<&'b str, usize>,
) -> usize {
    *node_names_to_index.entry(name).or_insert_with(|| {
        nodes.push(Node::empty());
        nodes.len() - 1
    })
}
