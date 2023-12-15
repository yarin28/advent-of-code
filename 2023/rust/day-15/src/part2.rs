use nom::character::complete::alpha1;

use crate::custom_error::AocError;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit0;
use nom::multi::separated_list1;
use nom::IResult;
#[derive(Debug)]
pub enum ActionType {
    Delete(Option<u32>),
    Add(Option<u32>),
}
#[derive(Debug)]
pub struct Value<'a> {
    lable: &'a str,
    action_type: ActionType,
}

pub fn parse_value(input: &str) -> IResult<&str, Value> {
    let (input, value) = alpha1(input)?;
    let (input, action_type) = alt((tag("="), tag("-")))(input)?;
    let (input, num) = digit0(input)?;
    let ret_num_option;
    if !num.is_empty() {
        ret_num_option = Some(num.parse::<u32>().expect("sould be va valid number{:num} "));
    } else {
        ret_num_option = None
    }
    let ret_action_type = match action_type {
        "-" => ActionType::Delete(ret_num_option),
        "=" => ActionType::Add(ret_num_option),
        _ => todo!(),
    };

    Ok((
        input,
        Value {
            lable: value,
            action_type: ret_action_type,
        },
    ))
}
pub fn parse(input: &str) -> IResult<&str, Vec<Value>> {
    let (input, values) = separated_list1(tag(","), parse_value)(input)?;
    Ok((input, values))
}

fn hash_str(value: &str) -> u32 {
    let mut return_value: u32 = 0;
    value.chars().for_each(|c| {
        return_value += c as u32;
        return_value *= 17;
        return_value %= 256;
    });
    return_value
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_input, values) = parse(input).expect("the parser should parse");
    let mut boxes: Vec<Vec<(&str, u32)>> = (0..256).map(|_| vec![]).collect();
    values.iter().for_each(|value| {
        let box_to_access = hash_str(value.lable);
        match value.action_type {
            ActionType::Delete(_num) => {
                if let Some(index_of_exsisting_lense) = boxes[box_to_access as usize]
                    .iter()
                    .position(|(lable, _value)| lable == &value.lable)
                {
                    boxes[box_to_access as usize].remove(index_of_exsisting_lense);
                }
            }
            ActionType::Add(num) => {
                if let Some(index_of_exsisting_lense) = boxes[box_to_access as usize]
                    .iter()
                    .position(|(lable, _value)| lable == &value.lable)
                {
                    boxes[box_to_access as usize][index_of_exsisting_lense] = (
                        value.lable,
                        num.expect(
                            "should be a valid num always inside an add instruction
                        ",
                        ),
                    );
                } else {
                    boxes[box_to_access as usize].push((
                        value.lable,
                        num.expect(
                            "should be a valid num always inside an add instruction
                        ",
                        ),
                    ))
                }
            }
        };
    });
    let foucusing_power = boxes
        .iter()
        .enumerate()
        .map(|(box_index, b)| {
            b.iter()
                .enumerate()
                .map(|(lens_index, (_lable, num))| {
                    (box_index as u32 + 1) * (lens_index as u32 + 1) * num
                })
                .sum::<u32>()
        })
        .sum::<u32>();
    Ok(foucusing_power.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7,";
        assert_eq!("145".to_string(), process(input)?);
        Ok(())
    }
}


