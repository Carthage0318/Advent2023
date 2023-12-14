use crate::data_structures::{Grid2D, GridPoint2D};
use crate::AdventErr::InputParse;
use crate::{parser, utils, AdventErr, AdventResult};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let mut grid = parser::as_grid2d_by_char(&mut input_file, |c| Space::try_from(c))?;

    // Part 1
    utils::part_header(1);
    part_1(&mut grid.clone());

    // Part 2
    utils::part_header(2);
    part_2(&mut grid);

    Ok(())
}

fn part_1(grid: &mut Grid2D<Space>) {
    grid.tilt_north();
    let north_support_load = grid.north_support_load();

    println!("Load on north support beams: {north_support_load}");
}

fn part_2(grid: &mut Grid2D<Space>) {
    const TOTAL_CYCLES: u32 = 1_000_000_000;
    let mut cache = Some(HashMap::new());

    let mut i: u32 = 0;
    while i < TOTAL_CYCLES {
        if cache.is_some() {
            if let Some(&last_seen) = cache.as_ref().unwrap().get(grid) {
                let cycle_len = i - last_seen;
                let skip_iterations = (TOTAL_CYCLES - i) / cycle_len;
                i += skip_iterations * cycle_len;
                cache = None;
            } else {
                cache.as_mut().unwrap().insert(grid.clone(), i);
            }
        }

        grid.spin_cycle();
        i += 1;
    }

    let north_support_load = grid.north_support_load();
    println!("Load on north support beams: {north_support_load}");
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Space {
    RoundRock,
    CubeRock,
    Empty,
}

impl Display for Space {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Space::RoundRock => write!(f, "O"),
            Space::CubeRock => write!(f, "#"),
            Space::Empty => write!(f, "."),
        }
    }
}

impl TryFrom<char> for Space {
    type Error = AdventErr;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'O' => Ok(Space::RoundRock),
            '#' => Ok(Space::CubeRock),
            '.' => Ok(Space::Empty),
            _ => Err(InputParse(format!("Unrecognized character '{value}'"))),
        }
    }
}

impl Grid2D<Space> {
    fn north_support_load(&self) -> u64 {
        self.rows()
            .enumerate()
            .map(|(row_num, row)| {
                let per_rock_load = (self.n_rows() - row_num) as u64;
                per_rock_load
                    * row
                        .iter()
                        .filter(|&&space| space == Space::RoundRock)
                        .count() as u64
            })
            .sum()
    }

    fn spin_cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn tilt_north(&mut self) {
        self.perform_tilt(
            GridPoint2D::new(0, 0),
            |point| Some(point.next_row()),
            |point| Some(point.next_column()),
        )
    }

    fn tilt_south(&mut self) {
        self.perform_tilt(
            GridPoint2D::new(self.n_rows() - 1, 0),
            |point| point.previous_row(),
            |point| Some(point.next_column()),
        )
    }

    fn tilt_east(&mut self) {
        self.perform_tilt(
            GridPoint2D::new(0, self.n_cols() - 1),
            |point| point.previous_column(),
            |point| Some(point.next_row()),
        )
    }

    fn tilt_west(&mut self) {
        self.perform_tilt(
            GridPoint2D::new(0, 0),
            |point| Some(point.next_column()),
            |point| Some(point.next_row()),
        )
    }

    fn perform_tilt(
        &mut self,
        stack_ptr_start: GridPoint2D,
        advance_in_aisle: impl Fn(GridPoint2D) -> Option<GridPoint2D>,
        to_next_aisle: impl Fn(GridPoint2D) -> Option<GridPoint2D>,
    ) {
        let mut aisle_start = Some(stack_ptr_start);

        'aisle: while let Some(mut stack_ptr) = aisle_start {
            if !self.in_bounds(stack_ptr) {
                break;
            }

            aisle_start = to_next_aisle(stack_ptr);

            'group: loop {
                // Advance stack ptr until it points to empty
                loop {
                    match self.get(stack_ptr) {
                        None => continue 'aisle,

                        Some(Space::Empty) => break,

                        _ => {
                            if let Some(next_stack_ptr) = advance_in_aisle(stack_ptr) {
                                stack_ptr = next_stack_ptr;
                            } else {
                                continue 'aisle;
                            }
                        }
                    }
                }

                let Some(mut search_ptr) = advance_in_aisle(stack_ptr) else {
                    continue 'aisle;
                };

                loop {
                    match self.get(search_ptr) {
                        None => continue 'aisle,

                        Some(Space::CubeRock) => {
                            stack_ptr = search_ptr;
                            continue 'group;
                        }

                        Some(Space::RoundRock) => {
                            *self.get_mut_unchecked(stack_ptr) = Space::RoundRock;
                            *self.get_mut_unchecked(search_ptr) = Space::Empty;
                            stack_ptr = advance_in_aisle(stack_ptr).unwrap();
                        }

                        Some(Space::Empty) => {}
                    }

                    if let Some(next_search_ptr) = advance_in_aisle(search_ptr) {
                        search_ptr = next_search_ptr
                    } else {
                        continue 'aisle;
                    }
                }
            }
        }
    }
}
