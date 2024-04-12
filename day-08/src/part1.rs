use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{
        alpha1, line_ending, multispace1, space1,
    },
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};

use crate::custom_error::AocError;

fn parse_directions(line: &str) -> IResult<&str, &str> {
    terminated(alpha1, multispace1)(line)
}

fn parse_row(
    line: &str,
) -> IResult<&str, (&str, (&str, &str))> {
    tuple((
        terminated(
            alpha1,
            tuple((space1, tag("="), space1, tag("("))),
        ),
        tuple((
            terminated(alpha1, tuple((tag(","), space1))),
            terminated(alpha1, tag(")")),
        )),
    ))(line)
}

fn parse_rows(
    input: &str,
) -> IResult<&str, Vec<(&str, (&str, &str))>> {
    separated_list1(line_ending, parse_row)(input)
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u32, AocError> {
    let (input, directions) =
        parse_directions(input).unwrap();

    let (_, rows) = parse_rows(input).unwrap();

    let mut key = "AAA";

    let h = HashMap::<&str, (&str, &str)>::from_iter(rows);

    let h = directions
        .chars()
        .cycle()
        .enumerate()
        .take_while(|(_, c)| {
            dbg!(c, key);
            if key == "ZZZ" {
                false
            } else {
                match c {
                    'L' => {
                        key = h.get(key).unwrap().0;
                        true
                    }
                    'R' => {
                        key = h.get(key).unwrap().1;
                        true
                    }
                    _ => {
                        panic!("Should only contain L or R")
                    }
                }
            }
        })
        .map(|(i, _)| i + 1)
        .last()
        .unwrap();
    Ok(h as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() -> miette::Result<()> {
        let input = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(2, process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_2() -> miette::Result<()> {
        let input = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(6, process(input)?);
        Ok(())
    }
}
