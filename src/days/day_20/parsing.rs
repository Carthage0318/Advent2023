use crate::days::day_20::types::{Module, NodeOutput, PulseType};
use crate::AdventErr::{Compute, InputParse};
use crate::AdventResult;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

const BROADCASTER_NAME: &str = "broadcaster";

pub(super) fn parse_input(input_file: &mut File) -> AdventResult<(Vec<Module>, usize)> {
    fn extract_module_identifier(line: &str) -> AdventResult<(Option<char>, &str, &str)> {
        let Some((identifier, outputs)) = line.split_once(" -> ") else {
            return Err(InputParse(format!("Failed to split line {line}")));
        };

        if identifier.len() < 2 {
            return Err(InputParse(format!(
                "Malformed module identifier: {identifier}"
            )));
        }

        if identifier == BROADCASTER_NAME {
            return Ok((None, identifier, outputs));
        }

        let type_char = identifier.chars().next().unwrap();
        let name = &identifier[type_char.len_utf8()..];
        Ok((Some(type_char), name, outputs))
    }

    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string)?;

    // Iterate once to allocate modules with their types
    let mut name_to_index = HashMap::new();
    let mut modules: Vec<_> = input_string
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (type_char, name, _) = extract_module_identifier(line)?;

            name_to_index.insert(name, i);

            Ok(match type_char {
                None => Module::new_broadcast(),
                Some('%') => Module::new_flip_flop(),
                Some('&') => Module::new_conjunction(),
                Some(c) => {
                    return Err(InputParse(format!(
                        "Unrecognized module type identifier '{c}'"
                    )))
                }
            })
        })
        .collect::<AdventResult<_>>()?;

    // Second pass - mark outputs
    for line in input_string.lines() {
        let (_, name, outputs) = extract_module_identifier(line)?;

        let this_index = name_to_index[name];
        for output_name in outputs.split(", ") {
            let output_index = *name_to_index.entry(output_name).or_insert_with(|| {
                modules.push(Module::new_untyped());
                modules.len() - 1
            });

            let input_id = modules[output_index].retain_input();

            modules[this_index].add_output(NodeOutput {
                to_node: output_index,
                input_id,
            })?
        }
    }

    let Some(&broadcast_index) = name_to_index.get(BROADCASTER_NAME) else {
        return Err(InputParse(String::from("Didn't find a broadcast module")));
    };

    // Retain an input on the broadcast module for the button
    modules[broadcast_index].retain_input();

    Ok((modules, broadcast_index))
}

impl Module {
    fn retain_input(&mut self) -> usize {
        match self {
            Self::FlipFlop(data) => {
                let result = data.input_count;
                data.input_count += 1;
                result
            }

            Self::Conjunction(data) => {
                data.cached_inputs.push(PulseType::Low);
                data.cached_inputs.len() - 1
            }

            Self::Broadcast(data) => {
                let result = data.input_count;
                data.input_count += 1;
                result
            }

            Self::Untyped(data) => {
                let result = data.input_count;
                data.input_count += 1;
                result
            }
        }
    }

    fn add_output(&mut self, node_output: NodeOutput) -> AdventResult<()> {
        match self {
            Self::FlipFlop(data) => data.outputs.push(node_output),
            Self::Conjunction(data) => data.outputs.push(node_output),
            Self::Broadcast(data) => data.outputs.push(node_output),
            Self::Untyped(_) => {
                return Err(Compute(String::from(
                    "Tried to add output to untyped module",
                )))
            }
        }

        Ok(())
    }
}
