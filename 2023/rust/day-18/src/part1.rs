use core::fmt;
use glam::i64vec2;
use nom::bytes::complete::take;
use nom::number::complete::hex_u32;
use std::iter;
use std::ops::Index;
use std::ops::IndexMut;

use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{hex_digit1, newline, space1};
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::Parser;
use nom::{
    branch::alt,
    character::complete::{self},
    IResult,
};

use crate::custom_error::AocError;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tile {
    Trench,
    Ground,
}
pub struct Ground {
    ground: Vec<Vec<Tile>>,
}
impl fmt::Display for Ground {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f).unwrap();
        self.ground.iter().for_each(|line| {
            line.iter().for_each(|tile| match tile {
                Tile::Ground => {
                    write!(f, ".").unwrap();
                }
                Tile::Trench => {
                    write!(f, "#").unwrap();
                }
            });
            writeln!(f, "").unwrap();
        });
        Ok(())
    }
}
impl fmt::Debug for Ground {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f).unwrap();
        self.ground.iter().for_each(|line| {
            line.iter().for_each(|tile| match tile {
                Tile::Ground => {
                    write!(f, ".").unwrap();
                }
                Tile::Trench => {
                    write!(f, "#").unwrap();
                }
            });
            writeln!(f, "").unwrap();
        });
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    #[default]
    None,
}
impl Direction {
    pub fn find_opposite_direction(&self) -> Self {
        match &self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::None => todo!(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Cursor {
    x: u32,
    y: u32,
}
impl Cursor {
    pub fn move_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::None => todo!(),
        }
    }
}
impl IndexMut<&Cursor> for Ground {
    fn index_mut(&mut self, cursor: &Cursor) -> &mut Self::Output {
        &mut self.ground[cursor.y as usize][cursor.x as usize]
    }
}
impl Index<&Cursor> for Ground {
    type Output = Tile;
    fn index(&self, cursor: &Cursor) -> &Self::Output {
        &self.ground[cursor.y as usize][cursor.x as usize]
    }
}
pub struct DigInstruction<'a> {
    direction: Direction,
    count: u32,
    color: &'a str,
}
pub fn dig_instruction_parse(input: &str) -> IResult<&str, DigInstruction> {
    let (input, direction) = alt((
        complete::char('D').map(|_| Direction::Down),
        complete::char('L').map(|_| Direction::Left),
        complete::char('R').map(|_| Direction::Right),
        complete::char('U').map(|_| Direction::Up),
    ))(input)?;
    let (input, count) = delimited(space1, complete::u32, space1)(input)?;
    let (input, color) = delimited(tag("(#"), hex_digit1, tag(")"))(input)?;

    Ok((
        input,
        DigInstruction {
            direction,
            count,
            color,
        },
    ))
}
pub fn parse(input: &str) -> IResult<&str, Vec<DigInstruction>> {
    let (input, dig_instructions) = separated_list1(newline, dig_instruction_parse)(input)?;
    Ok((input, dig_instructions))
}
pub fn find_max_direction(dig_instructions: &[DigInstruction], direction: Direction) -> u32 {
    let oppesite_direction = direction.find_opposite_direction();
    let mut max_length = 0;
    let mut current_length = 0;
    dig_instructions.iter().for_each(|dig_instruction| {
        if dig_instruction.direction == direction {
            current_length += dig_instruction.count;
        }
        if current_length > max_length {
            max_length = current_length;
        }
        if dig_instruction.direction == oppesite_direction {
            if !(current_length < dig_instruction.count) {
                current_length -= dig_instruction.count;
            }
        }
    });
    max_length + 1 // to prevent edge cases
}
pub fn insert_the_trenches(dig_instructions: &Vec<DigInstruction>, ground: &mut Ground) {
    let mut cursor = Cursor { x: 0, y: 0 };
    dig_instructions
        .iter()
        .fold(ground, |acc, dig_instruction| {
            iter::repeat(dig_instruction.direction)
                .take(dig_instruction.count.try_into().unwrap())
                .for_each(|direction| {
                    acc[&cursor] = Tile::Trench;
                    // println!(
                    //     "wants current cursor {:?} the direction that wants to move {:?}",
                    //     &cursor, direction
                    // );
                    cursor.move_direction(direction);
                });
            acc
        });
}

pub fn solve(instructions: &[DigInstruction]) -> u64 {
    let mut current = i64vec2(0, 0);
    let mut prev = i64vec2(0, 0);
    let mut count = 0;
    let mut sum = 0;
    for instruction in instructions {
        match instruction.direction {
            Direction::Up => current.y -= instruction.count as i64,
            Direction::Down => current.y += instruction.count as i64,
            Direction::Left => current.x -= instruction.count as i64,
            Direction::Right => current.x += instruction.count as i64,
            Direction::None => unreachable!("advent of code input should be valid"),
        }
        sum += current.x * prev.y - current.y * prev.x;
        count += instruction.count;
        prev = current;
    }
    (sum.abs() as u64) / 2 + count as u64 / 2 + 1
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (input, grid) = parse(input).expect("a valid parse");
    // let mut ground: Ground = Ground {
    //     ground: iter::repeat(
    //         iter::repeat(Tile::Ground)
    //             .take(
    //                 find_max_direction(&grid, Direction::Right)
    //                     .try_into()
    //                     .unwrap(),
    //             )
    //             .collect_vec(),
    //     )
    //     .take(
    //         find_max_direction(&grid, Direction::Down)
    //             .try_into()
    //             .unwrap(),
    //     )
    //     .collect_vec(),
    // };
    // insert_the_trenches(&grid, &mut ground);
    dbg!(solve(&grid));
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!("62", process(input)?);
        Ok(())
    }
}
