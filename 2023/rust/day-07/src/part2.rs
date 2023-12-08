#![allow(unused_variables)]
#![allow(dead_code)]
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::ops::Deref;

use itertools::{Itertools, Position};
use nom::bytes::complete::take_until;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::{character::complete::newline, IResult};

use crate::custom_error::AocError;
#[derive(Debug, Clone)]
pub struct Hand {
    cards: Vec<char>,
    bid_amount: u32,
    rank: (Type, (u32, u32, u32, u32, u32)),
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        dbg!("here");
        if self.rank.0 > other.rank.0 {
            return Ordering::Greater;
        } else if self.rank.0 < other.rank.0 {
            return Ordering::Less;
        } else {
            let mut order = Ordering::Equal;
            dbg!(&self.cards);
            self.cards
                .iter()
                .zip(other.cards.iter())
                .for_each(|(self_card, other_card)| {
                    dbg!(self_card, other_card);
                    if self_card > other_card {
                        order = Ordering::Greater;
                    } else if self_card < other_card {
                        order = Ordering::Less;
                    };
                });
            order
        }
    }
}
impl PartialOrd for Hand {
    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Less))
    }

    fn le(&self, other: &Self) -> bool {
        matches!(
            self.partial_cmp(other),
            Some(Ordering::Less | Ordering::Equal)
        )
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(
            self.partial_cmp(other),
            Some(Ordering::Greater | Ordering::Equal)
        )
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.rank.0 > other.rank.0 {
            return Some(Ordering::Greater);
        } else if self.rank.0 < other.rank.0 {
            return Some(Ordering::Less);
        } else {
            let mut order = Ordering::Equal;
            let card_type = create_card_type_map();
            self.cards
                .iter()
                .zip(other.cards.iter())
                .for_each(|(self_card, other_card)| {
                    let self_card_treanslated = &card_type
                        .get(self_card)
                        .expect("there is a key for the self_card");
                    let other_card_treanslated = &card_type
                        .get(other_card)
                        .expect("there is a key for the self_card");
                    if self_card_treanslated > other_card_treanslated && order == Ordering::Equal {
                        order = Ordering::Greater;
                        return;
                    } else if self_card_treanslated < other_card_treanslated
                        && order == Ordering::Equal
                    {
                        order = Ordering::Less;
                        return;
                    };
                });
            Some(order)
        }
    }
}
impl Hand {
    fn score_hand(hand: &str) -> (Type, (u32, u32, u32, u32, u32)) {
        use Type::*;

        let counts = hand.chars().counts();

        let values = if let Some(joker_count) = counts.get(&'J') {
            if *joker_count == 5 {
                "5".to_string()
            } else {
                counts
                    .iter()
                    .filter_map(|(key, value)| (key != &'J').then_some(value))
                    .sorted()
                    .with_position()
                    .map(|(position, value)| match position {
                        Position::Last | Position::Only => value + joker_count,
                        _ => *value,
                    })
                    .join("")
            }
        } else {
            counts.values().sorted().join("")
        };

        let hand_type = match values.deref() {
            "5" => FiveOfAKind,
            "14" => FourOfAKind,
            "23" => FullHouse,
            "113" => ThreeOfAKind,
            "122" => TwoPair,
            "1112" => OnePair,
            "11111" => HighCard,
            value => panic!("should never happen. Encountered `{}`", value),
        };
        let card_scores = hand
            .chars()
            .map(|card| match card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 1,
                'T' => 10,
                value => value.to_digit(10).unwrap(),
            })
            .collect_tuple()
            .unwrap();
        (hand_type, card_scores)
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.rank.0 == other.rank.0 {
            return false;
        }

        false
    }
}
impl Eq for Hand {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Type {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn hand(input: &str) -> IResult<&str, Hand> {
    let (input, cards) = take_until(" ")(input)?;
    let (input, _space) = space1(input)?;
    let (input, bid_amount) = nom::character::complete::u32(input)?;
    let cards = cards.chars().collect::<String>();
    Ok((
        input,
        Hand {
            cards: cards.chars().collect::<Vec<char>>(),
            rank: Hand::score_hand(&cards),
            bid_amount,
        },
    ))
}
fn parse(input: &str) -> IResult<&str, Vec<Hand>> {
    let (input, hands) = separated_list1(newline, hand)(input)?;
    Ok((input, hands))
}
fn create_card_type_map() -> BTreeMap<char, i32> {
    let mut card_type = BTreeMap::new();
    card_type.insert('A', 14);
    card_type.insert('K', 13);
    card_type.insert('Q', 12);
    card_type.insert('J', 1);
    card_type.insert('T', 10);
    card_type.insert('9', 9);
    card_type.insert('8', 8);
    card_type.insert('7', 7);
    card_type.insert('6', 6);
    card_type.insert('5', 5);
    card_type.insert('4', 4);
    card_type.insert('3', 3);
    card_type.insert('2', 2);
    card_type
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_input, mut hands) = parse(input).expect("valid input");
    hands.sort();
    dbg!(&hands);
    let mut res = 0;
    for (index, hand) in hands.iter().enumerate() {
        res += (index + 1) as u32 * hand.bid_amount
    }
    Ok(res.to_string())
    // Ok("ga".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("5905", process(input)?);
        Ok(())
    }
}


