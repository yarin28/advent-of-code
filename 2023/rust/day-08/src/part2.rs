use std::collections::BTreeMap;

use nom::{
    bytes::complete::{tag, take_till, take_until},
    character::{
        complete::{alphanumeric1, multispace1, newline, space1},
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
    let (input, (left, right)) = separated_pair(alphanumeric1, tag(", "), alphanumeric1)(input)?;
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
    let (input, navigation_instructions) = alphanumeric1(input)?;
    let (input, _space) = multispace1(input)?;
    let (input, network_nodes) = separated_list1(newline, network_node_parse)(input)?;
    let mut network_map = BTreeMap::new();
    network_nodes.iter().fold(&mut network_map, |acc, node| {
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
    let (_input, (navigation_instructions, network_nodes)) =
        parse(input).expect("should parse without any errors");
    let mut steps: u64 = 0;
    let mut instraction = navigation_instructions.chars().cycle();
    let mut node_names = network_nodes
        .iter()
        .filter(|(key, _node)| key.ends_with('A'))
        .map(|(key, _node)| key.as_str())
        .collect::<Vec<&str>>();
    while !node_names.iter().all(|name| name.ends_with('Z')) {
        match instraction.next() {
            Some('R') => node_names.iter_mut().for_each(|name| {
                *name = network_nodes
                    .get(*name)
                    .expect("there should be a valid entrence for {name:}")
                    .right
                    .as_str();
            }),
            Some('L') => node_names.iter_mut().for_each(|name| {
                *name = network_nodes
                    .get(*name)
                    .expect("there should be a valid entrence for {name:}")
                    .left
                    .as_str();
            }),
            _ => todo!(),
        }
        steps += 1;
        if steps % 1000000 == 0 {
            dbg!(steps);
        }
    }

    Ok(steps.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
