use crate::{parser, utils, AdventErr, AdventResult};
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::max;
use std::fs::File;

//noinspection DuplicatedCode
pub fn run(mut input_file: File) -> AdventResult<()> {
    let games = parser::as_vec_by_line(&mut input_file, line_parser)?;

    // Part 1
    utils::part_header(1);
    part_1(&games)?;

    // Part 2
    utils::part_header(2);
    part_2(&games)?;

    Ok(())
}

fn part_1(games: &[CubeGameInstance]) -> AdventResult<()> {
    let valid_game_sum: usize = games
        .iter()
        .filter(|game| {
            let most_seen = game.most_seen();
            most_seen.red <= 12 && most_seen.green <= 13 && most_seen.blue <= 14
        })
        .map(|game| game.id)
        .sum();

    println!("Sum of possible games: {valid_game_sum}");
    Ok(())
}

fn part_2(games: &[CubeGameInstance]) -> AdventResult<()> {
    let game_power_sum: u32 = games.iter().map(|game| game.most_seen().power()).sum();

    println!("Sum of game powers: {game_power_sum}");
    Ok(())
}

#[derive(Debug)]
struct CubeGameInstance {
    id: usize,
    reveals: Vec<CubeGroup>,
}

impl CubeGameInstance {
    fn most_seen(&self) -> CubeGroup {
        self.reveals
            .iter()
            .fold(CubeGroup::empty(), |acc, view| CubeGroup {
                red: max(acc.red, view.red),
                green: max(acc.green, view.green),
                blue: max(acc.blue, view.blue),
            })
    }
}

#[derive(Debug, Copy, Clone)]
struct CubeGroup {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeGroup {
    fn empty() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(r"Game (?<game_id>\d+): (?<views>.+)").unwrap();
    static ref GAME_REGEX: Regex = Regex::new(r"(?<count>\d+) (?<color>\w+)").unwrap();
}

fn line_parser(line: &str) -> AdventResult<CubeGameInstance> {
    let Some(caps) = LINE_REGEX.captures(line) else {
        return Err(AdventErr::InputParse(format!(
            "Failed to parse line:\n{line}"
        )));
    };

    let id = &caps["game_id"];
    let Ok(id) = id.parse() else {
        return Err(AdventErr::InputParse(format!(
            "Failed to parse game id '{id}' in line:\n{line}"
        )));
    };

    let views = &caps["views"];
    let reveals = views
        .split(';')
        .map(|view| {
            let mut cube_group = CubeGroup::empty();
            for caps in GAME_REGEX.captures_iter(view) {
                let count = &caps["count"];
                let Ok(count) = count.parse() else {
                    return Err(AdventErr::InputParse(format!(
                        "Failed to parse cube count '{count}' in game {id}. Line:\n{line}"
                    )));
                };

                match &caps["color"] {
                    "red" => cube_group.red = count,
                    "green" => cube_group.green = count,
                    "blue" => cube_group.blue = count,
                    s => {
                        return Err(AdventErr::InputParse(format!(
                            "Unknown color '{s}' in game {id}. Line:\n{line}"
                        )))
                    }
                };
            }

            Ok(cube_group)
        })
        .collect::<AdventResult<_>>()?;

    Ok(CubeGameInstance { id, reveals })
}
