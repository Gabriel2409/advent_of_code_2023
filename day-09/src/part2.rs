#[allow(unused_imports)]
use itertools::Itertools;
use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult,
};

use crate::custom_error::AocError;

fn parse_input(
    input: &str,
) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(
        line_ending,
        separated_list1(space1, complete::i32),
    )(input)
}

fn extrapolate(seq: &[i32]) -> i32 {
    let mut all_zeros = true;
    let next_seq: Vec<i32> = seq
        .iter()
        .tuple_windows()
        .map(|(t1, t2)| *t2 - *t1)
        .inspect(|c| {
            if *c != 0 {
                all_zeros = false;
            }
        })
        .collect();

    let first_val = *seq.first().unwrap_or(&0);

    if all_zeros {
        first_val
    } else {
        first_val - extrapolate(&next_seq)
    }
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<i32, AocError> {
    let (_, lines) = parse_input(input).unwrap();

    let sum: i32 =
        lines.iter().map(|l| extrapolate(l)).sum();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![10,  13,  16], 7 )]
    fn test_extrapolate(
        #[case] seq: Vec<i32>,
        #[case] expected: i32,
    ) {
        assert_eq!(extrapolate(&seq), expected);
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(2, process(input)?);
        Ok(())
    }
}
