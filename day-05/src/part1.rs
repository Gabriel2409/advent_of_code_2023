use nom::bytes::complete::take_till;
use nom::character::complete::{
    line_ending, multispace0, multispace1,
};
use nom::multi::many1;
use nom::{
    bytes::complete::tag,
    character::complete::{self, space0, space1},
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
    IResult, Parser,
};

use crate::custom_error::AocError;

#[derive(Debug)]
struct MapRange {
    source: u32,
    dest: u32,
    range: u32,
}

#[derive(Debug)]
struct Map {
    name: String,
    map_ranges: Vec<MapRange>,
}

fn get_seeds(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(
        tuple((tag("seeds:"), space0)),
        separated_list1(space1, complete::u32),
    )(input)
}

fn get_map_name(input: &str) -> IResult<&str, &str> {
    terminated(
        take_till(|c: char| c.is_whitespace()),
        tuple((space0, tag("map:"), line_ending)),
    )(input)
}

fn get_map_ranges(
    input: &str,
) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(
        line_ending,
        separated_list1(space1, complete::u32),
    )(input)
}
fn get_map(input: &str) -> IResult<&str, Map> {
    tuple((get_map_name, get_map_ranges))
        .map(|(name, ranges)| Map {
            name: name.to_string(),
            map_ranges: ranges
                .iter()
                .map(|nums| MapRange {
                    dest: nums[0],
                    source: nums[1],
                    range: nums[2],
                })
                .collect(),
        })
        .parse(input)
}

fn get_maps(input: &str) -> IResult<&str, Vec<Map>> {
    separated_list1(
        tuple((line_ending, line_ending)),
        get_map,
    )(input)
}

fn update_seed(
    seed: u32,
    map_range: &MapRange,
) -> (bool, u32) {
    println!(
        "{} {} {}",
        map_range.source, map_range.dest, seed
    );
    if (map_range.source <= seed)
        && (seed - map_range.source < map_range.range)
    {
        (
            true,
            seed - map_range.source + map_range.dest,
        )
    } else {
        (false, seed)
    }
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u32, AocError> {
    let (input, seeds) = get_seeds(input).unwrap();
    let res: IResult<&str, Vec<&str>> =
        many1(line_ending)(input);
    let (input, _) = res.unwrap();
    let (_, maps) = get_maps(input).unwrap();

    dbg!(&maps);

    let mini = seeds
        .into_iter()
        .map(|mut seed| {
            let mut updated;
            for m in &maps {
                for map_range in &m.map_ranges {
                    (updated, seed) =
                        update_seed(seed, map_range);
                    if updated {
                        break;
                    }
                }
            }

            seed
        })
        .min()
        .unwrap();

    Ok(mini)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("seeds: 45 8 78 5", vec![45,8,78,5])]
    #[case("seeds: 45 348 78    05\n", vec![45,348,78,5])]
    fn test_get_seeds(
        #[case] line: &str,
        #[case] expected: Vec<u32>,
    ) {
        let seeds = get_seeds(line).unwrap();
        assert_eq!(seeds.1, expected);
    }

    #[rstest]
    #[case(
        "soil-to-fertilizer    map:\n",
        "soil-to-fertilizer"
    )]
    fn test_get_map_name(
        #[case] line: &str,
        #[case] expected: &str,
    ) {
        let map_name = get_map_name(line).unwrap();
        assert_eq!(map_name.1, expected);
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(35, process(input)?);
        Ok(())
    }
}
