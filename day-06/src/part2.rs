use crate::custom_error::AocError;
use nom::character::complete::{
    alpha1, digit1, line_ending,
};
use nom::{
    bytes::complete::tag,
    character::complete::{self, space1},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult, Parser,
};

#[derive(Debug, PartialEq, Eq)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn is_winning_time(&self, t: u64) -> bool {
        let d = t * (self.time - t);
        d > self.distance
    }

    fn nb_combinations(&self) -> u64 {
        let mut ll = 0;
        let mut lr = self.time / 2;

        let mut lp;
        while ll < lr {
            lp = ll + (lr - ll) / 2;
            if self.is_winning_time(lp) {
                lr = lp - 1;
            } else {
                ll = lp + 1;
            }
        }

        let mut rl = self.time / 2;
        let mut rr = self.time;

        let mut rp;
        while rl < rr {
            rp = rl + (rr - rl) / 2;
            if self.is_winning_time(rp) {
                rl = rp + 1;
            } else {
                rr = rp - 1;
            }
        }

        rl - ll + 1
    }
}

fn parse_and_sum_numbers(
    input: &str,
) -> IResult<&str, u64> {
    let (remaining_input, numbers) =
        separated_list1(space1, digit1)(input)?; // Delegate initial parsing
    let parsed_num = numbers
        .join("")
        .parse::<u64>()
        .expect("Should be able to concat");
    Ok((remaining_input, parsed_num))
}

fn get_time_and_dist(
    input: &str,
) -> IResult<&str, Vec<u64>> {
    separated_list1(
        line_ending,
        preceded(
            tuple((alpha1, tag(":"), space1)),
            parse_and_sum_numbers,
        ),
    )
    .parse(input)
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u64, AocError> {
    let (_, time_and_dist) =
        get_time_and_dist(input).unwrap();
    if let [time, distance] = time_and_dist.as_slice() {
        let race = Race {
            time: *time,
            distance: *distance,
        };
        Ok(race.nb_combinations())
    } else {
        panic!("Should be able to build race")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Time: 35    87\nDistance:   8     97", vec![3587, 897])]
    fn test_get_time_and_dist(
        #[case] input: &str,
        #[case] expected: Vec<u64>,
    ) {
        let vecs = get_time_and_dist(input).unwrap();
        assert_eq!(vecs.1, expected);
    }
    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(71503, process(input)?);
        Ok(())
    }
}
