#![allow(unused_variables)]
#![allow(dead_code)]
use std::cmp::Ordering;
use std::collections::BTreeMap;

use itertools::Itertools;
use nom::bytes::complete::take_until;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::{character::complete::newline, IResult};

use crate::custom_error::AocError;
#[derive(Debug, Clone)]
pub struct Hand {
    cards: Vec<char>,
    hand_type: Option<Type>,
    bid_amount: u32,
    rank: Option<u32>,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        dbg!("here");
        if self.hand_type > other.hand_type {
            return Ordering::Greater;
        } else if self.hand_type < other.hand_type {
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
        if self.hand_type > other.hand_type {
            return Some(Ordering::Greater);
        } else if self.hand_type < other.hand_type {
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
    pub fn evalueate_hand_type(&mut self) {
        let card_symbol_and_amount = self.cards.iter().counts();
        let mut max_num = 0;
        let mut max_char = ' ';
        card_symbol_and_amount.iter().for_each(|(c, num)| {
            if *num > max_num && **c != 'J' {
                max_num = *num;
                max_char = **c;
            }
        });
        max_num += card_symbol_and_amount.get(&'J').unwrap_or(&0);
        if max_num == 5 {
            self.hand_type = Some(Type::FiveOfAKind);
        } else if max_num == 4 {
            self.hand_type = Some(Type::FourOfAKind);
        } else if max_num == 3 && card_symbol_and_amount.values().any(|num| *num == 2) {
            self.hand_type = Some(Type::FullHouse);
        } else if max_num == 3 && card_symbol_and_amount.values().all(|num| *num != 2) {
            self.hand_type = Some(Type::ThreeOfAKind);
        } else if max_num == 2
            && card_symbol_and_amount
                .values()
                .filter(|num| **num == 2)
                .count()
                == 2
        {
            self.hand_type = Some(Type::TwoPair);
        } else if max_num == 2
            && card_symbol_and_amount
                .values()
                .filter(|num| **num == 2)
                .count()
                == 1
        {
            self.hand_type = Some(Type::OnePair);
        } else {
            self.hand_type = Some(Type::HighCard);
        }
        dbg!(&self.cards, max_num, self.hand_type);
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_type == other.hand_type {
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
    let cards = cards.chars().collect::<Vec<char>>();
    Ok((
        input,
        Hand {
            cards,
            bid_amount,
            rank: None,
            hand_type: None,
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
    hands.iter_mut().for_each(|hand| hand.evalueate_hand_type());
    hands.sort();
    dbg!(&hands);
    let mut res = 0;
    for (index, hand) in hands.iter().enumerate() {
        res += (index + 1) as u32 * hand.bid_amount
    }
    Ok(res.to_string())
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


