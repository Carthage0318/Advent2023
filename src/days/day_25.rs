use crate::AdventErr::{Compute, InputParse};
use crate::{utils, AdventResult};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let mut node_edges = parse_input(&mut input_file)?;

    // Part 1
    utils::part_header(1);
    part_1(&mut node_edges)?;

    Ok(())
}

fn part_1(node_edges: &mut [HashMap<usize, usize>]) -> AdventResult<()> {
    let (side_1, side_2) = partition_by_cut(node_edges, 3)?;

    println!("Product of group sizes: {}", side_1 * side_2);

    Ok(())
}

/// Implements a modified Stoer-Wagner algorithm, which terminates as soon as it finds a cut
/// with the expected cut weight.
/// Returns the number of nodes present on each side of the cut
fn partition_by_cut(
    node_edges: &mut [HashMap<usize, usize>],
    expected_cut_weight: usize,
) -> AdventResult<(usize, usize)> {
    let total_node_count = node_edges.len();
    let mut node_sizes = vec![1_usize; total_node_count];
    let mut candidates = HashSet::with_capacity(total_node_count);
    for i in 0..node_edges.len() {
        candidates.insert(i);
    }

    let mut collected_nodes = HashSet::new();
    let mut outbound_edges = HashMap::new();
    while candidates.len() > 1 {
        collected_nodes.clear();
        outbound_edges.clear();

        let start = *candidates.iter().next().unwrap();
        collected_nodes.insert(start);
        for (&destination, &weight) in &node_edges[start] {
            outbound_edges.insert(destination, weight);
        }

        let mut last_node_added = start;
        let mut edges_to_remove = vec![];

        loop {
            let (&next_node_to_add, &weight_added) = outbound_edges
                .iter()
                .max_by_key(|(_, &weight)| weight)
                .unwrap();

            collected_nodes.insert(next_node_to_add);
            // Remove the edge from the collection to this node we just contracted
            outbound_edges.remove(&next_node_to_add);

            // Now incorporate all of its outbound edges
            for (&destination, &weight) in &node_edges[next_node_to_add] {
                if collected_nodes.contains(&destination) {
                    continue;
                }

                *outbound_edges.entry(destination).or_default() += weight;
            }

            if outbound_edges.is_empty() {
                // We've just added the last node.
                if weight_added == expected_cut_weight {
                    // That was it! Add up the sizes of all of the nodes, minus the one we just added.
                    let inside_count = collected_nodes
                        .iter()
                        .map(|&node_id| node_sizes[node_id])
                        .sum::<usize>()
                        - node_sizes[next_node_to_add];
                    let outside_count = total_node_count - inside_count;
                    return Ok((inside_count, outside_count));
                }

                // Merge the last node we added and this one
                // We do this by moving all data from the node we just added
                // to the one we did previously

                // Combine node sizes
                node_sizes[last_node_added] += node_sizes[next_node_to_add];
                node_sizes[next_node_to_add] = 0;

                // Combine edges - patch up both sides of these relationships
                edges_to_remove.extend(node_edges[next_node_to_add].drain());
                for (destination, weight) in edges_to_remove.drain(..) {
                    *node_edges[last_node_added].entry(destination).or_default() += weight;

                    node_edges[destination].remove(&next_node_to_add);
                    *node_edges[destination].entry(last_node_added).or_default() += weight;
                }

                // Invalidate removed node as a candidate for future start selection
                candidates.remove(&next_node_to_add);

                break;
            }

            last_node_added = next_node_to_add;
        }
    }

    Err(Compute(format!(
        "Completed search, but didn't find a cut of weight {expected_cut_weight}"
    )))
}

fn parse_input(input_file: &mut File) -> AdventResult<Vec<HashMap<usize, usize>>> {
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    let mut node_edges = vec![];
    let mut name_to_node_id = HashMap::new();

    for line in input.lines() {
        let line = line.trim();
        let Some((name, others)) = line.split_once(": ") else {
            return Err(InputParse(format!("Failed to split line:\n{line}")));
        };

        let source_node_id = *name_to_node_id.entry(name).or_insert_with(|| {
            node_edges.push(HashMap::new());
            node_edges.len() - 1
        });

        for destination in others.split_whitespace() {
            let destination_node_id = *name_to_node_id.entry(destination).or_insert_with(|| {
                node_edges.push(HashMap::new());
                node_edges.len() - 1
            });

            node_edges[source_node_id].insert(destination_node_id, 1);
            node_edges[destination_node_id].insert(source_node_id, 1);
        }
    }

    Ok(node_edges)
}
