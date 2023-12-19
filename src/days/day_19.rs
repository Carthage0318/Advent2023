use crate::days::day_19::types::{CopyRange, Destination, Part, PartRange, SortResult, Workflow};
use crate::AdventErr::Compute;
use crate::{utils, AdventResult};
use std::fs::File;

mod parsing;
mod types;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let (workflows, parts) = parsing::parse_input(&mut input_file)?;

    let start_index = workflows
        .iter()
        .position(|workflow| workflow.name == START_WORKFLOW_NAME)
        .ok_or_else(|| {
            Compute(format!(
                "Unable to find start workflow '{START_WORKFLOW_NAME}'"
            ))
        })?;

    // Part 1
    utils::part_header(1);
    part_1(&workflows, &parts, start_index)?;

    // Part 2
    utils::part_header(2);
    part_2(&workflows, start_index);

    Ok(())
}

const START_WORKFLOW_NAME: &str = "in";

fn part_1(workflows: &[Workflow], parts: &[Part], start_workflow_index: usize) -> AdventResult<()> {
    let accepted_part_rating_sum: u64 = parts
        .iter()
        .map(|&part| {
            sort_part(part, workflows, start_workflow_index).map(|result| match result {
                SortResult::Accepted => part.rating_sum(),
                SortResult::Rejected => 0,
            })
        })
        .sum::<AdventResult<_>>()?;

    println!("Sum of ratings of accepted parts: {accepted_part_rating_sum}");

    Ok(())
}

fn part_2(workflows: &[Workflow], start_workflow_index: usize) {
    let acceptable_ratings = total_acceptable_ratings(
        workflows,
        start_workflow_index,
        PartRange::new(CopyRange::new(1, 4001)),
    );

    println!("Acceptable combinations of ratings: {acceptable_ratings}");
}

fn sort_part(
    part: Part,
    workflows: &[Workflow],
    mut workflow_index: usize,
) -> AdventResult<SortResult> {
    Ok(loop {
        match workflows[workflow_index].apply_to(part)? {
            Destination::Accept => break SortResult::Accepted,
            Destination::Reject => break SortResult::Rejected,
            Destination::Workflow(next_index) => workflow_index = next_index,
        }
    })
}

fn total_acceptable_ratings(
    workflows: &[Workflow],
    workflow_index: usize,
    mut current_part: PartRange,
) -> u64 {
    let mut total = 0;
    for &rule in &workflows[workflow_index].rules {
        if let Some((deeper_part, destination)) = current_part.apply_rule(rule) {
            match destination {
                Destination::Accept => total += deeper_part.size(),
                Destination::Reject => { /* Do nothing */ }
                Destination::Workflow(next) => {
                    total += total_acceptable_ratings(workflows, next, deeper_part)
                }
            }
        }

        let Some(next_part) = current_part.apply_rule_inverse(rule) else {
            break;
        };
        current_part = next_part;
    }

    total
}
