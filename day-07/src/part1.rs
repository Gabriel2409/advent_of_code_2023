use nom::{
    character::complete::{
        self, alphanumeric1, line_ending, space1,
    },
    multi::separated_list1,
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
    tie_break: u32,
    bid: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // First compare by strength
        let strength_ordering =
            self.strength.cmp(&other.strength);

        // If strengths are equal, compare by tie_break
        if strength_ordering == std::cmp::Ordering::Equal {
            self.tie_break.cmp(&other.tie_break)
        } else {
            strength_ordering
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
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
        let mut tie_break: u32 = 0;

        let mut strength: u32 = 0;
        for (i, key) in cards.iter().enumerate() {
            let count = counter.entry(*key).or_insert(0);

            // first card adds value / 15, second over 225, etc...
            // This should act as a tie breaker for the strength
            // base 15 because we have less than 15 elements

            tie_break += *key * 15u32.pow((5 - i) as u32);

            *count += 1;
            match count {
                // pair gives 10 point
                2 => strength += 10,
                // 3 of a kind gives 100
                3 => strength += 90,
                // 4 of a kind gives 1000
                4 => strength += 900,
                // 5 of a kind gives 10000
                5 => strength += 9000,
                _ => {}
            }
        }

        Self {
            cards,
            strength,
            tie_break,
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

fn parse_lines(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(line_ending, parse_line)(input)
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u32, AocError> {
    let (_, mut hands) = parse_lines(input).unwrap();

    hands.sort();
    let sum = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as u32 * h.bid)
        .sum();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("32T3K 765", Hand{cards:vec![3,2,10,3,13], bid: 765, strength: 10, tie_break: 2413995})]
    #[case("33T3K 765", Hand{cards:vec![3,3,10,3,13], bid: 765, strength: 100, tie_break: 2464620})]
    #[case("33T3T 765", Hand{cards:vec![3,3,10,3,10], bid: 765, strength: 110, tie_break: 2464575})]
    #[case("3KT3T 765", Hand{cards:vec![3,13,10,3,10], bid: 765, strength: 20, tie_break: 2970825})]
    #[case("3333T 765", Hand{cards:vec![3,3,3,3,10], bid: 765, strength: 1000, tie_break: 2440950 })]
    #[case("33333 765", Hand{cards:vec![3,3,3,3,3], bid: 765, strength: 10000, tie_break: 2440845 })]
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
