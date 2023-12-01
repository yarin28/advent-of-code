use miette::Result;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let res = parse(input).unwrap();
    dbg!(res);
    Ok(res.to_string())
}

fn parse(input: &str) -> Result<i32> {
    let binding = input.lines().collect::<Vec<&str>>();
    let res = binding
        .iter()
        .map(|line| {
            line.chars()
                .filter(|c| c.to_string().parse::<i32>().ok().is_some())
                .collect()
        })
        .collect::<Vec<Vec<char>>>();
    let res: Vec<i32> = res
        .iter()
        .map(|line| {
            format!(
                "{}{}",
                line.iter().next().unwrap(),
                line.iter().last().unwrap()
            )
            .parse::<i32>()
            .unwrap()
        })
        .collect();

    Ok(res.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142".to_string(), process(input)?);
        Ok(())
    }
}
