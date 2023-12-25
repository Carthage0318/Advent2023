use crate::data_structures::{Grid2D, GridPoint2D};
use crate::AdventErr::{Compute, InputParse};
use crate::{math, parser, utils, AdventErr, AdventResult};
use std::collections::VecDeque;
use std::fs::File;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let sequences = parser::as_vec_by_line(&mut input_file, |line| {
        let values = line
            .split_whitespace()
            .map(|x| {
                x.parse::<i64>()
                    .map_err(|_| InputParse(format!("Failed to parse value '{x}'")))
            })
            .collect::<AdventResult<Vec<_>>>()?;

        Ok(Sequence { values })
    })?;

    let polynomials: Vec<_> = sequences
        .iter()
        .map(|sequence| Polynomial::try_from(sequence.values.as_slice()))
        .collect::<AdventResult<_>>()?;

    // Part 1
    utils::part_header(1);
    part_1(&sequences, &polynomials)?;

    // Part 2
    utils::part_header(2);
    part_2(&polynomials)?;

    Ok(())
}

fn part_1(sequences: &[Sequence], polynomials: &[Polynomial]) -> AdventResult<()> {
    assert_eq!(sequences.len(), polynomials.len());

    let next_values_sum: i64 = sequences
        .iter()
        .zip(polynomials)
        .map(|(sequence, polynomial)| {
            let next_index = sequence.values.len();
            Ok(polynomial.compute_at(next_index as i64))
        })
        .sum::<AdventResult<_>>()?;

    println!("Sum of next values: {next_values_sum}");
    Ok(())
}

fn part_2(polynomials: &[Polynomial]) -> AdventResult<()> {
    let previous_values_sum: i64 = polynomials
        .iter()
        .map(|polynomial| Ok(polynomial.compute_at(-1_i64)))
        .sum::<AdventResult<_>>()?;

    println!("Sum of previous values: {previous_values_sum}");
    Ok(())
}

#[derive(Debug, Clone)]
struct Sequence {
    values: Vec<i64>,
}

#[derive(Debug, Clone)]
struct Polynomial {
    coefficients: Vec<f64>,
}

impl Polynomial {
    fn compute_at(&self, x: i64) -> i64 {
        self.coefficients
            .iter()
            .enumerate()
            .map(|(power, &coefficient)| coefficient * big_pow(x as i128, power as u32) as f64)
            .sum::<f64>()
            .round() as i64
    }
}

// Scary that we need this... maybe this isn't the "right" solution.
fn big_pow(base: i128, exponent: u32) -> i128 {
    let mut result = 1;
    for _ in 0..exponent {
        result *= base;
    }
    result
}

impl TryFrom<&[i64]> for Polynomial {
    type Error = AdventErr;

    fn try_from(value: &[i64]) -> Result<Self, Self::Error> {
        let Some(degree) = polynomial_degree(value) else {
            return Err(Compute(format!("Failed to find degree for {value:?}")));
        };

        let variable_count = degree + 1;

        // to be used for Gauss-Jordan
        let mut b_values: Vec<_> = value[0..variable_count]
            .iter()
            .rev()
            .map(|&x| x as f64)
            .collect();

        let mut a_values = Grid2D::new(variable_count, variable_count, 0_f64);

        for row_num in 0..a_values.n_rows() - 1 {
            let sequence_index = a_values.n_rows() - 1 - row_num;
            let row = a_values.row_mut_unchecked(row_num);
            for (power, cell) in row.iter_mut().rev().enumerate() {
                *cell = big_pow(sequence_index as i128, power as u32) as f64;
            }
        }

        // Last row is all 0's. To avoid 0^0, just set it manually.
        let bottom_right_cell = a_values.get_mut_unchecked(GridPoint2D::new(
            a_values.n_rows() - 1,
            a_values.n_cols() - 1,
        ));
        *bottom_right_cell = 1_f64;

        math::gauss_jordan(&mut a_values, &mut b_values)?;

        b_values.reverse();

        Ok(Polynomial {
            coefficients: b_values,
        })
    }
}

fn polynomial_degree(sequence: &[i64]) -> Option<usize> {
    if sequence.len() == 0 {
        return None;
    }

    if sequence.iter().all(|&x| x == 0) {
        return Some(0);
    }

    let mut sequence = VecDeque::from_iter(sequence.iter().copied());
    let mut degree = 0;

    loop {
        let n = sequence.len() - 1;
        if n == 0 {
            return None;
        }

        for _ in 0..n {
            let diff = sequence[1] - sequence[0];
            sequence.pop_front();
            sequence.push_back(diff);
        }

        // Remove last element from this iteration
        sequence.pop_front();

        if sequence.iter().all(|&x| x == 0) {
            return Some(degree);
        }

        degree += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomial_degree() {
        let cases = [
            (vec![0, 0, 0, 0], 0),
            (vec![5, 5], 0),
            (vec![1, 2, 3], 1),
            (vec![1, 2, 3, 4, 5], 1),
            (vec![1, 2, 4, 7], 2),
            (vec![1, 2, 4, 7, 11], 2),
            (vec![10, 13, 16, 21, 30, 45], 3),
        ];

        for (sequence, expected) in cases {
            assert_eq!(
                expected,
                polynomial_degree(&sequence).unwrap(),
                "polynomial_degree({:?})",
                &sequence
            );
        }

        assert_eq!(None, polynomial_degree(&[]), "polynomial degree of empty");
    }
}
