use crate::custom_error::AocError;
use anyhow::Result;
use glam::I64Vec3;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete,
    character::complete::{newline, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

#[derive(Debug)]
pub struct Hail {
    starting_position: I64Vec3,
    direction: I64Vec3,
}

#[derive(Debug)]
pub struct IntersectionPoint {
    x: f64,
    y: f64,
}
impl IntersectionPoint {
    pub fn is_inside_bounderies(&self) -> bool {
        let lower_boundary: f64 = 200000000000000f64; // TODO: must change before actual run.
        let upper_boundary: f64 = 400000000000000f64;
        println!(
            "x:{:?} y:{:?} -> {:?}",
            self.x,
            self.y,
            self.x >= lower_boundary
                && self.y >= lower_boundary
                && self.x <= upper_boundary
                && self.y <= upper_boundary
        );
        self.x >= lower_boundary
            && self.y >= lower_boundary
            && self.x <= upper_boundary
            && self.y <= upper_boundary
    }
}

impl Hail {
    pub fn get_intersection(&self, other: &Self) -> Option<IntersectionPoint> {
        if self.check_if_slopes_are_the_same(other) {
            return None;
        }

        let (a1, b1, c1) = (
            self.direction.y as f64,
            -self.direction.x as f64,
            (self.direction.y * self.starting_position.x
                - self.direction.x * self.starting_position.y) as f64,
        );
        let (a2, b2, c2) = (
            other.direction.y as f64,
            -other.direction.x as f64,
            (other.direction.y * other.starting_position.x
                - other.direction.x * other.starting_position.y) as f64,
        );
        let (x, y);
        x = (c1 * b2 - c2 * b1) / (a1 * b2 - a2 * b1);
        y = (c2 * a1 - c1 * a2) / (a1 * b2 - a2 * b1);
        let intersection_point = IntersectionPoint { x, y };
        if self.is_intersection_is_in_past(&intersection_point)
            || other.is_intersection_is_in_past(&intersection_point)
        {
            return None;
        }
        Some(intersection_point)
    }
    pub fn is_intersection_is_in_past(&self, intersection_point: &IntersectionPoint) -> bool {
        !((intersection_point.x - self.starting_position.x as f64) * self.direction.x as f64
            >= 0f64
            || (intersection_point.y - self.starting_position.y as f64) * self.direction.y as f64
                >= 0f64)
    }

    pub fn check_if_slopes_are_the_same(&self, other: &Self) -> bool {
        let (a1, b1, c1) = (
            self.direction.y,
            -self.direction.x,
            (self.direction.y * self.starting_position.x
                - self.direction.x * self.starting_position.y),
        );
        let (a2, b2, c2) = (
            other.direction.y,
            -other.direction.x,
            (other.direction.y * other.starting_position.x
                - other.direction.x * other.starting_position.y),
        );
        a1 * b2 == b1 * a2
    }
}

pub fn vec_parse(input: &str) -> IResult<&str, I64Vec3> {
    let (input, x) = complete::i64(input)?;
    let (input, _) = terminated(tag(","), space1)(input)?;
    let (input, y) = complete::i64(input)?;
    let (input, _) = terminated(tag(","), space1)(input)?;
    let (input, z) = complete::i64(input)?;
    Ok((input, I64Vec3::new(x, y, z)))
}
pub fn hail_parse(input: &str) -> IResult<&str, Hail> {
    let (input, (starting_position, direction)) =
        separated_pair(vec_parse, delimited(space1, tag("@"), space1), vec_parse)(input)?;
    Ok((
        input,
        Hail {
            starting_position,
            direction,
        },
    ))
}
pub fn parse(input: &str) -> IResult<&str, Vec<Hail>> {
    let (input, hails) = separated_list1(newline, hail_parse)(input)?;
    Ok((input, hails))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let upper_boundary: f64 = 200000000000000f64; // TODO: must change before actual run.
    let lower_boundary: f64 = 400000000000000f64;

    let (input, hails) = parse(input).expect("correct_input");
    let intersection_points: Vec<IntersectionPoint> = hails
        .iter()
        .tuple_combinations()
        .map(|(hail1, hail2)| hail1.get_intersection(hail2))
        .flatten()
        .collect();
    let amout_of_intersection_points_inside_boundaries = intersection_points
        .iter()
        .filter(|intersection_point| intersection_point.is_inside_bounderies())
        .count();
    let res = amout_of_intersection_points_inside_boundaries;
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        assert_eq!("2", process(input,)?);
        Ok(())
    }
}
