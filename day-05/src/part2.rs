use itertools::Itertools;
use std::ops::Range;

use nom::bytes::complete::take_till;
use nom::character::complete::line_ending;
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
    source_range: Range<u64>,
    dest_range: Range<u64>,
}

impl MapRange {
    fn from_vec(v: &[u64]) -> Self {
        if v.len() != 3 {
            panic!("Expected correct input")
        }
        MapRange {
            // NOTE: in input v[1] + v[2] actually overflows for u32
            // I could use part1 impl to avoid storing the end and only store the
            // len instead to avoid overflows
            source_range: v[1]..(v[1] + v[2]),
            dest_range: v[0]..(v[0] + v[2]),
        }
    }

    // First part of the tuple is ranges that were not updated and second are the
    // updated ranges
    fn translate_range(
        &self,
        r: Range<u64>,
    ) -> (Vec<Range<u64>>, Vec<Range<u64>>) {
        if r.end <= self.source_range.start
            || r.start >= self.source_range.end
        {
            (vec![r], Vec::new())
        } else {
            let mut fr = Vec::new();
            let mut fr2 = Vec::new();

            if r.start < self.source_range.start {
                fr.push(r.start..self.source_range.start);
            }

            let real_start =
                r.start.max(self.source_range.start);
            let real_end = r.end.min(self.source_range.end);

            let updated_range = (real_start
                - self.source_range.start
                + self.dest_range.start)
                ..(real_end - self.source_range.start
                    + self.dest_range.start);
            fr2.push(updated_range);

            if r.end > self.source_range.end {
                fr.push(self.source_range.end..r.end);
            }

            (fr, fr2)
        }
    }
    fn translate_ranges(
        &self,
        ranges: Vec<Range<u64>>,
    ) -> (Vec<Range<u64>>, Vec<Range<u64>>) {
        let mut not_transformed = Vec::new();
        let mut transformed = Vec::new();
        for r in ranges {
            let (nt, t) = self.translate_range(r);
            not_transformed.extend(nt);
            transformed.extend(t);
        }
        (not_transformed, transformed)
    }
}

#[derive(Debug)]
struct Map {
    name: String,
    map_ranges: Vec<MapRange>,
}

impl Map {
    fn translate_ranges(
        &self,
        mut ranges: Vec<Range<u64>>,
    ) -> Vec<Range<u64>> {
        let mut updated = Vec::new();
        for mr in &self.map_ranges {
            let (new_ranges, cur_updated) =
                mr.translate_ranges(ranges);
            ranges = new_ranges;
            updated.extend(cur_updated);
        }

        ranges.extend(updated);
        ranges
    }
}

fn get_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(
        tuple((tag("seeds:"), space0)),
        separated_list1(space1, complete::u64),
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
) -> IResult<&str, Vec<Vec<u64>>> {
    separated_list1(
        line_ending,
        separated_list1(space1, complete::u64),
    )(input)
}
fn get_map(input: &str) -> IResult<&str, Map> {
    tuple((get_map_name, get_map_ranges))
        .map(|(name, ranges)| Map {
            name: name.to_string(),
            map_ranges: ranges
                .iter()
                .map(|nums| MapRange::from_vec(nums))
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

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u64, AocError> {
    let (input, seeds) = get_seeds(input).unwrap();
    let res: IResult<&str, Vec<&str>> =
        many1(line_ending)(input);
    let (input, _) = res.unwrap();
    let (_, maps) = get_maps(input).unwrap();

    let mut ranges = seeds
        .into_iter()
        .tuples()
        .map(|(r1, r2)| r1..(r1 + r2))
        .collect::<Vec<_>>();

    for m in maps {
        ranges = m.translate_ranges(ranges)
    }

    let mini =
        ranges.into_iter().map(|r| r.start).min().unwrap();

    Ok(mini)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case( 0..4, 6..9, 15..18, (vec![0..4], vec![]) )]
    #[case( 45..48, 6..9, 15..18, (vec![45..48],vec![]) )]
    #[case( 5..10, 6..9, 15..18, (vec![5..6,  9..10], vec![15..18]) )]
    #[case( 5..8, 6..9, 15..18,( vec![5..6], vec![15..17]) )]
    #[case( 5..9, 6..9, 15..18,( vec![5..6], vec![15..18]) )]
    #[case( 6..9, 6..9, 15..18,( vec![], vec![15..18]) )]
    #[case( 7..8, 6..9, 15..18,( vec![ ], vec![16..17]) )]
    #[case( 8..11, 6..9, 15..18,( vec![  9..11], vec![17..18]) )]
    fn test_translate_range(
        #[case] initial_range: Range<u64>,
        #[case] source_range: Range<u64>,
        #[case] dest_range: Range<u64>,
        #[case] expected: (
            Vec<Range<u64>>,
            Vec<Range<u64>>,
        ),
    ) {
        let mr = MapRange {
            source_range,
            dest_range,
        };
        let fr = mr.translate_range(initial_range);
        assert_eq!(fr, expected);
    }

    #[test]
    fn test_map_translate_ranges() {
        let ranges = vec![5..9, 15..18];

        let my_map = Map {
            name: "mymap".to_string(),
            map_ranges: vec![
                MapRange {
                    source_range: 8..10,
                    dest_range: 19..21,
                },
                MapRange {
                    source_range: 16..19,
                    dest_range: 35..38,
                },
            ],
        };

        let ranges = my_map.translate_ranges(ranges);
        let expected = vec![5..8, 15..16, 19..20, 35..37];
        assert_eq!(ranges, expected);
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
        assert_eq!(46, process(input)?);
        Ok(())
    }
}
