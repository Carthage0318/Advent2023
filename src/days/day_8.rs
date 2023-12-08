use crate::AdventErr::Compute;
use crate::{math, utils, AdventResult};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use types::{Cycle, Instruction, MapSpec, Node};

mod parsing;
mod types;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let (instructions, nodes, map_spec) = parsing::parse_input(&mut input_file)?;

    // Part 1
    utils::part_header(1);
    part_1(&instructions, &nodes, &map_spec);

    // Part 2
    utils::part_header(2);
    part_2(&instructions, &nodes, &map_spec)?;

    Ok(())
}

fn part_1(instructions: &[Instruction], nodes: &[Node], map_spec: &MapSpec) {
    let mut steps: u64 = 0;
    let mut current = map_spec.aaa;
    while current != map_spec.zzz {
        let instruction_index = steps as usize % instructions.len();
        let node = &nodes[current];
        current = match instructions[instruction_index] {
            Instruction::Left => node.left,
            Instruction::Right => node.right,
        };
        steps += 1;
    }

    println!("Steps required: {steps}");
}

fn part_2(instructions: &[Instruction], nodes: &[Node], map_spec: &MapSpec) -> AdventResult<()> {
    let cycles: Vec<_> = map_spec
        .start_nodes
        .iter()
        .map(|&start| find_end_cycles(instructions, nodes, start, &map_spec.end_nodes))
        .collect();

    let min_steps = cycles
        .iter()
        .multi_cartesian_product()
        .filter_map(|cycle_set| find_convergence(&cycle_set))
        .min()
        .ok_or_else(|| Compute(String::from("Unable to find cycle convergence")))?;

    println!("Steps required: {min_steps}");

    Ok(())
}

fn find_end_cycles(
    instructions: &[Instruction],
    nodes: &[Node],
    start_node_id: usize,
    end_node_ids: &[usize],
) -> Vec<Cycle> {
    #[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
    struct VisitDef {
        node_id: usize,
        modular_step: usize,
    }

    #[derive(Debug, Copy, Clone)]
    struct VisitInfo {
        last_step: u64,
    }

    let mut is_end_node = vec![false; nodes.len()];
    for &end_node_id in end_node_ids {
        is_end_node[end_node_id] = true;
    }

    let mut visited: HashMap<VisitDef, VisitInfo> = HashMap::new();
    let mut cycles = Vec::new();
    let mut stop: Option<VisitDef> = None; // Halt once we reach this node at this modular step
    let mut steps: u64 = 0;
    let mut current_node_id = start_node_id;

    loop {
        let modular_step = steps as usize % instructions.len();
        let visit_def = VisitDef {
            node_id: current_node_id,
            modular_step,
        };

        if stop.is_some_and(|x| x == visit_def) {
            // We've found all possible cycles.
            break;
        }

        // Mark the current location
        if let Some(info) = visited.get_mut(&visit_def) {
            // We've been here before!
            if stop.is_none() {
                stop = Some(visit_def);
            }

            // Add a cycle record if this is an end node
            if is_end_node[current_node_id] {
                cycles.push(Cycle {
                    length: steps - info.last_step,
                    offset: info.last_step,
                });
            }
        } else {
            visited.insert(visit_def, VisitInfo { last_step: steps });
        }

        // Advance
        let instruction = instructions[modular_step];
        let node = &nodes[current_node_id];
        current_node_id = match instruction {
            Instruction::Left => node.left,
            Instruction::Right => node.right,
        };

        steps += 1;
    }

    cycles
}

fn find_convergence(cycles: &[&Cycle]) -> Option<u64> {
    let remainders: Vec<_> = cycles.iter().map(|cycle| cycle.offset).collect();

    let moduli: Vec<_> = cycles.iter().map(|cycle| cycle.length).collect();

    let base = math::chinese_remainder_theorem(&remainders, &moduli)?;
    let &min_start = remainders.iter().max().unwrap(); // If empty, would have returned above.

    if base >= min_start {
        Some(base)
    } else {
        Some(base + math::lcm(&remainders).unwrap()) // If empty, would have returned above
    }
}
