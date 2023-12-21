use crate::days::day_20::types::{Module, NodeOutput, PulseType, SentPulse};
use crate::math;
use crate::AdventErr::Compute;
use crate::{utils, AdventResult};
use std::collections::VecDeque;
use std::fs::File;

mod parsing;
mod types;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let (mut modules, broadcast_index) = parsing::parse_input(&mut input_file)?;

    // Part 1
    utils::part_header(1);
    part_1(&mut modules, broadcast_index);

    utils::part_header(2);
    part_2(&modules, broadcast_index)?;

    Ok(())
}

fn part_1(modules: &mut [Module], broadcast_index: usize) {
    let (total_low, total_high) = (0..1000)
        .map(|_| press_button(modules, broadcast_index))
        .reduce(|(left_low, left_high), (right_low, right_high)| {
            (left_low + right_low, left_high + right_high)
        })
        .unwrap();

    println!(
        "Product of low and high pulses sent: {}",
        total_low * total_high
    );
}

fn part_2(modules: &[Module], broadcast_index: usize) -> AdventResult<()> {
    let Module::Broadcast(ref broadcast) = modules[broadcast_index] else {
        return Err(Compute(String::from(
            "Broadcast index led to a non-broadcast node",
        )));
    };

    let (cycles, offsets): (Vec<_>, Vec<_>) = broadcast
        .outputs
        .iter()
        .map(|output| process_counter(modules, output.to_node))
        .collect::<AdventResult<Vec<_>>>()?
        .into_iter()
        .unzip();

    let base = math::chinese_remainder_theorem(&offsets, &cycles).ok_or_else(|| {
        Compute(String::from(
            "Failed to compute solution for set of counters",
        ))
    })?;
    let min_value = *offsets.iter().max().unwrap(); // If no values were present, the above line would have returned.

    let first_finish_pulse = if base > min_value {
        base
    } else {
        base + math::lcm(&cycles).unwrap()
    };

    println!("Button presses required for low pulse to rx: {first_finish_pulse}");

    Ok(())
}

fn press_button(modules: &mut [Module], broadcast_index: usize) -> (u64, u64) {
    let mut low_signals = 0;
    let mut high_signals = 0;

    let mut queue = VecDeque::new();
    queue.push_back(SentPulse {
        pulse_type: PulseType::Low,
        destination: NodeOutput {
            to_node: broadcast_index,
            input_id: 0,
        },
    });

    while let Some(pulse) = queue.pop_front() {
        match pulse.pulse_type {
            PulseType::Low => low_signals += 1,
            PulseType::High => high_signals += 1,
        }

        match modules[pulse.destination.to_node] {
            Module::FlipFlop(ref mut flip_flop) => {
                match pulse.pulse_type {
                    PulseType::Low => {
                        // Flip state, send pulses
                        flip_flop.is_on = !flip_flop.is_on;

                        let new_pulse_type = if flip_flop.is_on {
                            PulseType::High
                        } else {
                            PulseType::Low
                        };
                        for output in &flip_flop.outputs {
                            queue.push_back(output.as_sent_pulse(new_pulse_type));
                        }
                    }
                    PulseType::High => { /* Do nothing */ }
                }
            }

            Module::Conjunction(ref mut conjunction) => {
                // First, update cached value
                conjunction.cached_inputs[pulse.destination.input_id] = pulse.pulse_type;

                let new_pulse_type = if conjunction
                    .cached_inputs
                    .iter()
                    .all(|&pulse_type| pulse_type == PulseType::High)
                {
                    PulseType::Low
                } else {
                    PulseType::High
                };
                for output in &conjunction.outputs {
                    queue.push_back(output.as_sent_pulse(new_pulse_type));
                }
            }

            Module::Broadcast(ref broadcast) => {
                // Send same pulse type to all outputs
                for output in &broadcast.outputs {
                    queue.push_back(output.as_sent_pulse(pulse.pulse_type));
                }
            }

            Module::Untyped(_) => { /* Do Nothing */ }
        }
    }

    (low_signals, high_signals)
}

/// This method relies on a pre-determined structure for the puzzle input.
/// (This was discovered by inspection - see the graphviz files in this day's directory.)
/// The modules which are signaled from the broadcaster are the first in a chain of flip-flops
/// which form a counter.
/// Some of these flip flops are wired to an conjunction,
/// which causes that counter to "fire" when its target value
/// (defined by the bit pattern of which flip-flops send pulses to the conjunction) is reached.
/// The outputs of the conjunction are
/// (1) an inverter which is then connected to the final conjunction, and
/// (2) the inverse bit pattern of the target value, plus one.
/// The latter resets the counter for the next loop.
///
/// In practice, it appears that the counters are all reset to 0,
/// but for completeness, we handle the possibility that it might reset to a positive value,
/// thus shortening the cycle length.
fn process_counter(modules: &[Module], mut module_index: usize) -> AdventResult<(u64, u64)> {
    let mut module_to_bit = vec![None; modules.len()];
    let mut bit_to_module = vec![];
    let mut conjunction_index = None;

    // Build the bit arrays.

    loop {
        let Module::FlipFlop(ref flip_flop) = modules[module_index] else {
            return Err(Compute(String::from(
                "Initial module index was not a flip flop",
            )));
        };

        module_to_bit[module_index] = Some(bit_to_module.len());
        bit_to_module.push(module_index);

        let mut next_index = None;
        for output in &flip_flop.outputs {
            match &modules[output.to_node] {
                Module::FlipFlop(_) => match next_index {
                    None => next_index = Some(output.to_node),
                    Some(_) => {
                        return Err(Compute(String::from(
                            "A flip flop in the counter has multiple flip flop outputs",
                        )))
                    }
                },
                Module::Conjunction(_) => {
                    match conjunction_index {
                        None => conjunction_index = Some(output.to_node),
                        Some(value) => {
                            if value != output.to_node {
                                return Err(Compute(String::from("Unexpected structure - multiple conjunctions connected to counter")));
                            }
                        }
                    }
                }
                _ => {
                    return Err(Compute(String::from(
                        "Unexpected structure - unexpected module type found connected to counter",
                    )))
                }
            }
        }

        match next_index {
            None => break,
            Some(next_index) => module_index = next_index,
        }
    }

    let Some(conjunction_index) = conjunction_index else {
        return Err(Compute(String::from(
            "Failed to find conjunction in counter",
        )));
    };

    let trigger_value: u64 = bit_to_module
        .iter()
        .enumerate()
        .filter(|(_, &flip_flop_index)| {
            let Module::FlipFlop(ref flip_flop) = modules[flip_flop_index] else {
                return false; // invalid flip_flip index - this should never happen
            };

            flip_flop
                .outputs
                .iter()
                .any(|output| output.to_node == conjunction_index)
        })
        .map(|(i, _)| 2_u64.pow(i as u32))
        .sum();

    let Module::Conjunction(ref conjunction) = modules[conjunction_index] else {
        return Err(Compute(String::from(
            "Incorrect conjunction index - this shouldn't happen",
        )));
    };
    let reset_pattern: u64 = conjunction
        .outputs
        .iter()
        .filter_map(|output| {
            let module_index = output.to_node;
            let Some(bit_position) = module_to_bit[module_index] else {
                return None;
            };
            Some(2_u64.pow(bit_position as u32))
        })
        .sum();

    let post_reset = (trigger_value + reset_pattern) % 2_u64.pow(bit_to_module.len() as u32);
    let loop_length = if trigger_value >= post_reset {
        trigger_value - post_reset
    } else {
        trigger_value + 2_u64.pow(bit_to_module.len() as u32) - post_reset
    };

    Ok((loop_length, trigger_value))
}
