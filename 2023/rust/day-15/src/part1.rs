use crate::custom_error::AocError;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::multi::separated_list1;
use nom::IResult;

pub fn parse_value(input: &str) -> IResult<&str, &str> {
    let (input, value) = take_until(",")(input)?;
    Ok((input, value))
}
pub fn parse(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, values) = separated_list1(tag(","), parse_value)(input)?;
    Ok((input, values))
}

fn hash_str(value: &str) -> u32 {
    let mut return_value: u32 = 0;
    value.chars().clone().into_iter().for_each(|c| {
        return_value += c as u32;
        return_value = return_value * 17;
        return_value = return_value % 256
    });
    dbg!(return_value);
    return_value
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_input, values) = parse(input).expect("the parser should parse");
    dbg!(&values);
    let res: u32 = values.into_iter().map(|value| hash_str(value)).sum();
    Ok(res.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7,";
        assert_eq!("1320".to_string(), process(input)?);
        Ok(())
    }
}
