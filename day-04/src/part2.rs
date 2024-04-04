use std::collections::{HashMap, HashSet};

use crate::custom_error::AocError;
use itertools::Itertools;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u32, AocError> {
    let mut counter = HashMap::<usize, u32>::new();
    let sum: u32 = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let mut char_iter =
                line.chars().skip_while(|c| *c != ':');

            char_iter.next();

            let winning_nos = char_iter
                .by_ref()
                .take_while(|c| *c != '|')
                .group_by(|c| c.is_ascii_digit())
                .into_iter()
                .filter_map(|(cond, group)| match cond {
                    true => Some(
                        group
                            .collect::<String>()
                            .parse::<u32>()
                            .unwrap(),
                    ),
                    false => None,
                })
                .collect::<HashSet<_>>();

            let played_nos = char_iter
                .group_by(|c| c.is_ascii_digit())
                .into_iter()
                .filter_map(|(cond, group)| match cond {
                    true => Some(
                        group
                            .collect::<String>()
                            .parse::<u32>()
                            .unwrap(),
                    ),
                    false => None,
                })
                .collect::<HashSet<_>>();

            let intersection = winning_nos
                .intersection(&played_nos)
                .collect::<HashSet<_>>();

            let current_count =
                *counter.get(&i).unwrap_or(&1);

            for j in 1..=intersection.len() {
                let a = i + j;
                let count = counter.entry(a).or_insert(1);
                *count += current_count;
            }
            current_count
        })
        .sum();
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(30, process(input)?);
        Ok(())
    }
}
