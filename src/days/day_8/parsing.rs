use crate::days::day_8::types::{Instruction, MapSpec, Node};
use crate::AdventErr::InputParse;
use crate::{parser, AdventResult};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

lazy_static! {
    static ref LINE_REGEX: Regex =
        Regex::new(r"(?<this>[\w]+)\s+=\s+\((?<left>[\w]+),\s+(?<right>[\w]+)\)").unwrap();
}

pub fn parse_input<'a>(
    input_file: &mut File,
) -> AdventResult<(Vec<Instruction>, Vec<Node>, MapSpec)> {
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    let Some((instructions, nodes_str)) = input.split_once("\n\n") else {
        return Err(InputParse(String::from("Failed to split input")));
    };

    let instructions = parser::as_vec_by_char(instructions, Instruction::try_from)?;

    let mut nodes = Vec::new();
    let mut node_names_to_index = HashMap::new();

    let mut aaa = None;
    let mut zzz = None;
    let mut start_nodes = Vec::new();
    let mut end_nodes = Vec::new();

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

        if aaa.is_none() && this_name == "AAA" {
            aaa = Some(this);
        } else if zzz.is_none() && this_name == "ZZZ" {
            zzz = Some(this);
        }

        if this_name.ends_with('A') {
            start_nodes.push(this);
        } else if this_name.ends_with('Z') {
            end_nodes.push(this);
        }
    }

    Ok((
        instructions,
        nodes,
        MapSpec {
            aaa: aaa.ok_or_else(|| InputParse(String::from("Failed to find start node")))?,
            zzz: zzz.ok_or_else(|| InputParse(String::from("Failed to find end node")))?,
            start_nodes,
            end_nodes,
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
