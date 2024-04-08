use crate::custom_error::AocError;
use itertools::Itertools;
use nom::character::complete::{alpha1, line_ending};
use nom::{
    bytes::complete::tag,
    character::complete::{self, space1},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult, Parser,
};

#[derive(Debug, PartialEq, Eq)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn nb_combinations(&self) -> u32 {
        let mut tot = 0;
        for t in 1..self.time {
            let d = t * (self.time - t);
            if d > self.distance {
                tot += 1
            };
        }
        tot
    }
}

fn get_time_and_dist_vecs(
    input: &str,
) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(
        line_ending,
        preceded(
            tuple((alpha1, tag(":"), space1)),
            separated_list1(space1, complete::u32),
        ),
    )(input)
}

fn get_races(time_and_dist_vecs: &[Vec<u32>]) -> Vec<Race> {
    if time_and_dist_vecs.len() != 2 {
        panic!("Should have exactly one vector for time and one for distance")
    }

    if let [time_vec, dist_vec] = time_and_dist_vecs {
        time_vec
            .iter()
            .zip_eq(dist_vec)
            .map(|(time, distance)| Race {
                time: *time,
                distance: *distance,
            }) // Access the first element of each subvector
            .collect()
    } else {
        panic!("Should be able to construct the races")
    }
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u32, AocError> {
    let (_, time_and_dist_vecs) =
        get_time_and_dist_vecs(input).unwrap();
    let races = get_races(&time_and_dist_vecs);

    let prod = races
        .into_iter()
        .map(|race| race.nb_combinations())
        .product();

    Ok(prod)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Time: 35    87\nDistance:   8     97", vec![vec![35,87], vec![8,97]])]
    fn test_get_time_and_dist_vecs(
        #[case] input: &str,
        #[case] expected: Vec<Vec<u32>>,
    ) {
        let vecs = get_time_and_dist_vecs(input).unwrap();
        assert_eq!(vecs.1, expected);
    }

    #[rstest]
    #[case(vec![vec![35,87], vec![8,97]], vec![Race{time:35, distance:8}, Race{time:87, distance: 97}])]
    fn test_get_races(
        #[case] input: Vec<Vec<u32>>,
        #[case] expected: Vec<Race>,
    ) {
        let races = get_races(&input);
        assert_eq!(races, expected);
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(288, process(input)?);
        Ok(())
    }
}
