use nom::{
    character::complete::{self, alphanumeric1, space1},
    sequence::{terminated, tuple},
    IResult, Parser,
};
use std::{collections::HashMap, sync::OnceLock};

use crate::custom_error::AocError;

// not sure if this is the right way
pub fn letter_map() -> &'static HashMap<char, u32> {
    static INSTANCE: OnceLock<HashMap<char, u32>> =
        OnceLock::new();

    INSTANCE.get_or_init(|| {
        HashMap::from([
            ('2', 2),
            ('3', 3),
            ('3', 3),
            ('4', 4),
            ('5', 5),
            ('6', 6),
            ('7', 7),
            ('8', 8),
            ('9', 9),
            ('T', 10),
            ('J', 11),
            ('Q', 12),
            ('K', 13),
            ('A', 14),
        ])
    })
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<u32>,
    strength: u32,
    bid: u32,
}

impl Hand {
    fn from_letters_and_bid(
        letters: &str,
        bid: u32,
    ) -> Self {
        let cards = letters
            .chars()
            .map(|c| *letter_map().get(&c).unwrap())
            .collect::<Vec<_>>();

        let mut counter = HashMap::<u32, u32>::new();

        let mut strength = 0;
        for key in &cards {
            let count = counter.entry(*key).or_insert(0);

            *count += 1;
            match count {
                2 => strength += 10,
                3 => strength += 90,
                4 => strength += 900,
                5 => strength += 9000,
                _ => {}
            }
        }

        // 12345=>0
        // 11234=>10
        // 11223=>20
        // 11123=>100
        // 11122=>110
        // 11112=>1000
        // 11111=>10000

        Self {
            cards,
            strength,
            bid,
        }
    }
}

fn parse_line(line: &str) -> IResult<&str, Hand> {
    tuple((
        terminated(alphanumeric1, space1),
        complete::u32,
    ))
    .map(|(letters, bid)| {
        Hand::from_letters_and_bid(letters, bid)
    })
    .parse(line)
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u32, AocError> {
    parse_line(input).unwrap();
    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("32T3K 765", Hand{cards:vec![3,2,10,3,13], bid: 765, strength:10})]
    #[case("33T3K 765", Hand{cards:vec![3,3,10,3,13], bid: 765, strength:100})]
    #[case("33T3T 765", Hand{cards:vec![3,3,10,3,10], bid: 765, strength:110})]
    #[case("3KT3T 765", Hand{cards:vec![3,13,10,3,10], bid: 765, strength:20})]
    #[case("3333T 765", Hand{cards:vec![3,3,3,3,10], bid: 765, strength:1000})]
    #[case("33333 765", Hand{cards:vec![3,3,3,3,3], bid: 765, strength:10000})]
    fn test_parse_line(
        #[case] line: &str,
        #[case] expected: Hand,
    ) {
        let (_, parsed) = parse_line(line).unwrap();
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(6440, process(input)?);
        Ok(())
    }
}
