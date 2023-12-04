use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{line_ending, multispace0, multispace1, newline, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

use crate::custom_error::AocError;
use nom::character::complete::char;
use nom::character::complete::u32;

#[derive(Debug)]
pub struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    owned_numbers: Vec<u32>,
}
fn get_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, space) = space1(input)?;
    let (input, nums) = separated_list1(space1, u32)(input)?;
    // dbg!(&nums);
    let (input, space) = space1(input)?;
    dbg!(&space);
    Ok((input, nums))
}
fn get_card(input: &str) -> IResult<&str, Card> {
    dbg!(input);
    let (input, id) = preceded(tag("Card "), u32)(input)?;
    dbg!(id);
    let (input, _colon) = tag(":")(input)?;
    dbg!(_colon);
    let (input, (winning_numbers, owned_numbers)) =
        separated_pair(get_numbers, char('|'), get_numbers)(input)?;
    Ok((
        input,
        Card {
            id,
            winning_numbers,
            owned_numbers,
        },
    ))
}
fn parse(input: &str) -> Vec<Card> {
    let (input, cards) = separated_list1(line_ending, get_card)(input).unwrap();
    cards
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let cards = parse(input);
    dbg!(cards);
    Ok(12)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(13, process(input)?);
        Ok(())
    }
}
