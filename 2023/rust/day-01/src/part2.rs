use miette::Result;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let res = parse(input).unwrap();
    Ok(res.to_string())
}

fn parse(input: &str) -> Result<i32> {
    let binding = input.lines().collect::<Vec<&str>>();
    let res = binding.iter().map(|line| process_line(line)).sum::<i32>();

    Ok(res)
}

fn process_line(line: &str) -> i32 {
    let digits_in_words: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut first = None;
    let mut last = 0;
    let chars = line.as_bytes();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_ascii_digit() {
            first = first.or(Some((c - b'0') as i32));
            last = (c - b'0') as i32;
        } else {
            for (j, k) in digits_in_words.iter().enumerate() {
                if chars[i..].starts_with(k.as_bytes()) {
                    first = first.or(Some((j + 1) as i32));
                    last = (j + 1) as i32;
                }
            }
        }
        i += 1;
    }
    (first.unwrap() * 10) + last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281".to_string(), process(input)?);
        Ok(())
    }
}
