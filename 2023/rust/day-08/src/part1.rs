use std::collections::BTreeMap;

use nom::{
    bytes::complete::{tag, take_till, take_until},
    character::{
        complete::{alpha1, multispace1, newline, space1},
        is_alphabetic,
    },
    multi::{separated_list0, separated_list1},
    sequence::{separated_pair, tuple},
    IResult,
};

use crate::custom_error::AocError;

#[derive(Debug, Clone)]
pub struct NetworkNodeInside {
    left: String,
    right: String,
}

#[derive(Debug, Clone)]
pub struct NetworkNode {
    id: String,
    left: String,
    right: String,
}

pub fn network_node_parse(input: &str) -> IResult<&str, NetworkNode> {
    let (input, id) = take_until(" = ")(input)?;
    let (input, _space) = tuple((space1, tag("="), space1, tag("(")))(input)?;
    let (input, (left, right)) = separated_pair(alpha1, tag(", "), alpha1)(input)?;
    let (input, _closing_bracket) = tag(")")(input)?;
    Ok((
        input,
        NetworkNode {
            id: id.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        },
    ))
}
pub fn parse(input: &str) -> IResult<&str, (String, BTreeMap<String, NetworkNodeInside>)> {
    let (input, navigation_instructions) = alpha1(input)?;
    let (input, _space) = multispace1(input)?;
    let (input, network_nodes) = separated_list1(newline, network_node_parse)(input)?;
    let mut network_map = BTreeMap::new();
    network_nodes
        .iter()
        .fold(&mut network_map, |mut acc, node| {
            acc.insert(
                node.id.to_string(),
                NetworkNodeInside {
                    left: node.left.clone(),
                    right: node.right.clone(),
                },
            );
            acc
        });
    Ok((input, (navigation_instructions.to_string(), network_map)))
}
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (input, (navigation_instructions, mut network_nodes)) =
        parse(input).expect("should parse without any errors");
    // dbg!(&navigation_instructions, &network_nodes);
    let mut instraction = navigation_instructions.chars().cycle();
    let steps = 0;
    let mut node_name = network_nodes
        .first_entry()
        .expect("there should be a valid entry")
        .key()
        .to_string();
    let mut count = 0;
    // let mut node_name = node_name.clone();
    while *node_name != "ZZZ".to_string() {
        match instraction.next() {
            Some('R') => {
                let next_node_id = &network_nodes
                    .get(&(&node_name).clone().to_string())
                    .expect("there should be a valid entry")
                    .right;

                let next_node_id = next_node_id.to_string();
                node_name = (next_node_id.clone());
            }
            Some('L') => {
                let next_node_id = &network_nodes
                    .get(&(&node_name).clone().to_string())
                    .expect("there should be a valid entry")
                    .left;

                let next_node_id = next_node_id.to_string();
                node_name = (next_node_id.clone());
            }
            _ => todo!(),
        }
        // dbg!(&node_name);
        count += 1;
        // dbg!(count);
    }
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
