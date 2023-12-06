use nom::{
    bytes::complete::tag,
    character::complete::{newline, space0, space1},
    multi::fold_many1,
    sequence::terminated,
    IResult,
};

use crate::custom_error::AocError;
use nom::character::complete::u32;

#[derive(Debug)]
pub struct Race {
    time: u32,
    distance: u32,
}
impl Race {
    pub fn get_amount_of_better_times(&self) -> u32 {
        (1..self.time)
            .map(|index| {
                let dist = index * (self.time - index);
                if dist > self.distance {
                    1
                } else {
                    0
                }
            })
            .sum::<u32>()
    }
}
pub fn parse(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, _time) = tag("Time:")(input)?;
    let (input, _space) = space1(input)?;
    dbg!(_time);
    let (input, times) = fold_many1(
        terminated(u32, space0),
        Vec::new,
        |mut acc: Vec<_>, item| {
            acc.push(item);
            acc
        },
    )(input)?;
    let (input, _newline) = newline(input)?;
    let (input, _time) = tag("Distance:")(input)?;
    let (input, _space) = space1(input)?;
    let (input, distances) = fold_many1(
        terminated(u32, space0),
        Vec::new,
        |mut acc: Vec<_>, item| {
            acc.push(item);
            acc
        },
    )(input)?;
    let res = times
        .iter()
        .enumerate()
        .map(|(index, time)| Race {
            time: *time,
            distance: *distances.get(index).unwrap(),
        })
        .collect::<Vec<Race>>();
    Ok((input, res))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_index, races) = parse(input).expect("the parser should work");
    let amount_of_bigger: Vec<u32> = races
        .iter()
        .map(|race| race.get_amount_of_better_times())
        .collect();
    let res: u32 = amount_of_bigger.iter().product();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("288".to_string(), process(input)?);
        Ok(())
    }
}
