use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::newline,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

use crate::custom_error::AocError;
use nom::character::complete::u32;

#[derive(Debug)]
pub struct Game {
    id: u8,
    subset_of_cubes: Vec<Vec<Cube>>,
}
#[derive(Debug)]
pub struct Cube {
    color: Color,
    amount: usize,
}
#[derive(Debug, PartialEq)]
pub enum Color {
    Red = 12,
    Green = 13,
    Blue = 14,
}
#[derive(Debug)]
pub struct SumOfColorsInElfGrab {
    red: usize,
    green: usize,
    blue: usize,
}
fn get_cube(input: &str) -> IResult<&str, Cube> {
    let (input, _whitespace) = tag(" ")(input)?;
    let (input, amount_of_cubes) = u32(input)?;
    let (input, _whitespace) = tag(" ")(input)?;
    let (input, color_of_cube) = alt((tag("green"), tag("red"), tag("blue")))(input)?;
    Ok((
        input,
        Cube {
            color: match color_of_cube {
                "green" => Color::Green,
                "blue" => Color::Blue,
                "red" => Color::Red,
                &_ => todo!(),
            },
            amount: amount_of_cubes as usize,
        },
    ))
}
fn get_handful_of_cubes(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, handful_of_cubes) = separated_list1(tag(","), get_cube)(input)?;
    Ok((input, handful_of_cubes))
}
fn get_all_handful_of_cubes(input: &str) -> IResult<&str, Vec<Vec<Cube>>> {
    let (input, _whitespace) = take(1 as usize)(input)?;
    let (input, all_handful_of_cubes) = separated_list1(tag(";"), get_handful_of_cubes)(input)?;
    Ok((input, all_handful_of_cubes))
}
fn get_game_id(input: &str) -> IResult<&str, u32> {
    Ok(preceded(tag("Game "), u32)(input)?)
}

fn get_game(input: &str) -> IResult<&str, Game> {
    let (input, game_id) = get_game_id(input)?;
    let (input, game) = get_all_handful_of_cubes(input).unwrap();
    Ok((
        input,
        Game {
            id: game_id as u8,
            subset_of_cubes: game,
        },
    ))
}
fn parse(input: &str) -> Vec<Game> {
    let (input, game) = separated_list1(newline, get_game)(input).unwrap();
    game
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let get_amount_of_colored_cubes_in_handful_of_elf =
        |subset_of_cubes: &Vec<Cube>, color: Color| {
            subset_of_cubes
                .iter()
                .filter(|cube| cube.color == color)
                .collect::<Vec<&Cube>>()
                .into_iter()
                .map(|cube| cube.amount)
                .sum()
        };

    let games = parse(input);
    let result: usize = games
        .iter()
        .map(|game| {
            let binding = game
                .subset_of_cubes
                .iter()
                .map(|subset_of_cubes| SumOfColorsInElfGrab {
                    red: get_amount_of_colored_cubes_in_handful_of_elf(subset_of_cubes, Color::Red),
                    green: get_amount_of_colored_cubes_in_handful_of_elf(
                        subset_of_cubes,
                        Color::Green,
                    ),
                    blue: get_amount_of_colored_cubes_in_handful_of_elf(
                        subset_of_cubes,
                        Color::Blue,
                    ),
                })
                .map(|elf_grab| {
                    // dbg!(Color::Red as usize);
                    !(elf_grab.red > Color::Red as usize
                        || elf_grab.green > Color::Green as usize
                        || elf_grab.blue > Color::Blue as usize)
                })
                .collect::<Vec<bool>>();
            // dbg!(&binding);
            // binding.iter().find(|b| **b);
            let res = binding.iter().min();
            // dbg!(res);
            match res {
                Some(b) => match *b {
                    true => game.id,
                    false => 0,
                },
                None => 0,
            }
        })
        .collect::<Vec<u8>>()
        .iter()
        .map(|num| *num as usize)
        .sum();
    dbg!(result);
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8".to_string(), process(input)?);
        Ok(())
    }
}
