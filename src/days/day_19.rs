use crate::days::day_19::types::{Destination, Part, SortResult, Workflow};
use crate::AdventErr::Compute;
use crate::{utils, AdventResult};
use std::fs::File;

mod parsing;
mod types;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let (workflows, parts) = parsing::parse_input(&mut input_file)?;

    // Part 1
    utils::part_header(1);
    part_1(&workflows, &parts)?;

    Ok(())
}

const START_WORKFLOW_NAME: &str = "in";

fn part_1(workflows: &[Workflow], parts: &[Part]) -> AdventResult<()> {
    let start_index = workflows
        .iter()
        .position(|workflow| workflow.name == START_WORKFLOW_NAME)
        .ok_or_else(|| {
            Compute(format!(
                "Unable to find start workflow '{START_WORKFLOW_NAME}'"
            ))
        })?;

    let accepted_part_rating_sum: u64 = parts
        .iter()
        .map(|&part| {
            sort_part(part, workflows, start_index).map(|result| match result {
                SortResult::Accepted => part.rating_sum(),
                SortResult::Rejected => 0,
            })
        })
        .sum::<AdventResult<_>>()?;

    println!("Sum of ratings of accepted parts: {accepted_part_rating_sum}");

    Ok(())
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
