use std::collections::BTreeMap;
use std::collections::HashSet;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{digit1, line_ending, multispace0, multispace1, newline, space0, space1},
    multi::{fold_many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::custom_error::AocError;
use nom::character::complete::char;
use nom::character::complete::u32;

#[derive(Debug)]
pub struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    owned_numbers: HashSet<u32>,
}

impl Card {
    fn get_score(&self) -> u32 {
        let amount_of_intersections = self
            .owned_numbers
            .intersection(&self.winning_numbers)
            .count() as u32;
        if amount_of_intersections == 0 {
            return 0;
        };
        2u32.pow(amount_of_intersections - 1)
    }
    fn amount_of_intersections(&self) -> u32 {
        self.owned_numbers
            .intersection(&self.winning_numbers)
            .count() as u32
    }
}
fn set(input: &str) -> IResult<&str, HashSet<u32>> {
    fold_many1(
        terminated(u32, space0),
        HashSet::new,
        |mut acc: HashSet<_>, item| {
            acc.insert(item);
            acc
        },
    )(input)
}
fn get_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _space) = space1(input)?;
    let (input, nums) = separated_list1(space1, u32)(input)?;
    let (input, _space2) = space1(input)?;
    Ok((input, nums))
}
fn get_card(input: &str) -> IResult<&str, Card> {
    let (input, id) = delimited(
        tuple((tag("Card"), space1)),
        digit1,
        tuple((tag(":"), space1)),
    )(input)?;
    let id = id.parse().unwrap();
    let (input, (winning_numbers, owned_numbers)) =
        separated_pair(set, tuple((tag("|"), space1)), set)(input)?;
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
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let cards = parse(input);
    let cards_and_amount_of_winning_numbers = cards
        .iter()
        .map(|card| (card.id, card.amount_of_intersections()))
        .collect::<BTreeMap<u32, u32>>();
    let mut cards_and_amount_of_copies = cards
        .iter()
        .map(|card| (card.id, 1))
        .collect::<BTreeMap<u32, u32>>();
    let result = cards_and_amount_of_winning_numbers.iter().enumerate().fold(
        cards_and_amount_of_copies.clone(),
        |mut acc, (_, (index, card_score))| {
            dbg!(index);
            let to_add = *acc.get(index).unwrap();
            for i in (index + 1)..((index + *card_score + 1) as u32) {
                acc.entry(i as u32).and_modify(|value| {
                    *value += to_add;
                });
            }
            acc
        },
    );
    dbg!(&cards_and_amount_of_winning_numbers);
    dbg!(&cards_and_amount_of_copies);
    let res: u32 = result.values().sum();
    // dbg!(cards);
    Ok(res.to_string())
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
        assert_eq!("30".to_string(), process(input)?);
        Ok(())
    }
}


