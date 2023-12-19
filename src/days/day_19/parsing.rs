use crate::days::day_19::types::{Destination, Part, Rule, Workflow};
use crate::AdventErr::InputParse;
use crate::{parser, AdventResult};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub(super) fn parse_input(input_file: &mut File) -> AdventResult<(Vec<Workflow>, Vec<Part>)> {
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    let Some((workflows, parts)) = input.split_once("\n\n") else {
        return Err(InputParse(String::from(
            "Failed to split input into sections",
        )));
    };

    Ok((parse_workflows(workflows)?, parse_parts(parts)?))
}

lazy_static! {
    static ref WORKFLOW_OUTER_REGEX: Regex =
        Regex::new(r"(?<name>[A-Za-z]+)\{(?<rules>.+)\}").unwrap();
    static ref RULE_REGEX: Regex = Regex::new(
        r"(?<category>[xmas])(?<comparison>[<>])(?<value>\d+):(?<destination>[A-Za-z]+)"
    )
    .unwrap();
    static ref DESTINATION_REGEX: Regex = Regex::new("^[A-Za-z]+$").unwrap();
}

fn parse_workflows(section: &str) -> AdventResult<Vec<Workflow>> {
    let mut name_to_index = HashMap::new();
    let mut workflows = Vec::new();

    for line in section.lines() {
        let Some(caps) = WORKFLOW_OUTER_REGEX.captures(line) else {
            return Err(InputParse(format!(
                "Failed to parse workflow line:\n{line}"
            )));
        };

        let name = caps.name("name").unwrap().as_str();
        let index = get_workflow_index(name, &mut workflows, &mut name_to_index);
        let rules = caps.name("rules").unwrap().as_str();
        for rule in rules.split(',') {
            let rule = if let Some(caps) = RULE_REGEX.captures(rule) {
                let category = caps["category"].chars().next().unwrap().try_into()?;
                let comparison = caps["comparison"].chars().next().unwrap();
                let value = caps["value"]
                    .parse()
                    .map_err(|_| InputParse(format!("Unable to parse value from rule '{rule}'")))?;
                let destination = caps.name("destination").unwrap().as_str();
                let destination = get_destination(destination, &mut workflows, &mut name_to_index);

                match comparison {
                    '<' => Rule::LessThan(category, value, destination),
                    '>' => Rule::GreaterThan(category, value, destination),
                    _ => {
                        return Err(InputParse(format!(
                            "Unrecognized comparison '{comparison}'"
                        )))
                    }
                }
            } else if DESTINATION_REGEX.is_match(rule) {
                Rule::Jump(get_destination(rule, &mut workflows, &mut name_to_index))
            } else {
                return Err(InputParse(format!(
                    "Failed to parse rule from string '{rule}'"
                )));
            };

            workflows[index].rules.push(rule);
        }
    }

    Ok(workflows)
}

fn get_destination<'a, 'b: 'a>(
    name: &'b str,
    workflows: &mut Vec<Workflow>,
    name_to_index: &mut HashMap<&'a str, usize>,
) -> Destination {
    match name {
        "A" => Destination::Accept,
        "R" => Destination::Reject,
        name => Destination::Workflow(get_workflow_index(name, workflows, name_to_index)),
    }
}

fn get_workflow_index<'a, 'b: 'a>(
    name: &'b str,
    workflows: &mut Vec<Workflow>,
    name_to_index: &mut HashMap<&'a str, usize>,
) -> usize {
    *name_to_index.entry(name).or_insert_with(|| {
        workflows.push(Workflow::empty(name.to_string()));
        workflows.len() - 1
    })
}

lazy_static! {
    static ref PART_REGEX: Regex =
        Regex::new(r"\{x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)\}").unwrap();
}

fn parse_parts(section: &str) -> AdventResult<Vec<Part>> {
    parser::as_vec_by_line_from_str(section, |line| {
        let Some(caps) = PART_REGEX.captures(line) else {
            return Err(InputParse(format!("Failed to match part line:\n{line}")));
        };

        let x = caps["x"]
            .parse()
            .map_err(|_| InputParse(format!("Failed to parse x value from part '{line}'")))?;
        let m = caps["m"]
            .parse()
            .map_err(|_| InputParse(format!("Failed to parse m value from part '{line}'")))?;
        let a = caps["a"]
            .parse()
            .map_err(|_| InputParse(format!("Failed to parse a value from part '{line}'")))?;
        let s = caps["s"]
            .parse()
            .map_err(|_| InputParse(format!("Failed to parse s value from part '{line}'")))?;

        Ok(Part { x, m, a, s })
    })
}
