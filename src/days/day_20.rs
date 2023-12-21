use crate::days::day_20::types::{Module, NodeOutput, PulseType, SentPulse};
use crate::math;
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
    part_2();

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

fn part_2() {
    // This part was solved by manual inspection.
    // See the graphviz file in this day's folder.
    // The graph forms a set of 4 counters,
    // each made of a sequence of flip-flops.
    // Some of these flip flops are wired to an conjunction,
    // which causes that counter to "fire" when its target value
    // (defined by the bit pattern of which flip-flops send pulses to the conjunction)
    // is reached.
    // The outputs of the conjunction are
    // (1) an inverter which is then connected to the final conjunction, and
    // (2) the inverse bit pattern of the target value, plus one.
    // The latter resets the counter to 0 for the next loop.

    let target_values: [u64; 4] = [
        0b111100100101,
        0b111101000011,
        0b111110111011,
        0b111110100001,
    ];
    let first_finish_pulse = math::lcm(&target_values).unwrap();

    println!("Button presses required for low pulse to rx: {first_finish_pulse}");
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
