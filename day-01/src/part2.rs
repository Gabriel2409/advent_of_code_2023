use std::collections::HashMap;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u32, AocError> {
    let number_map: HashMap<&str, &str> = HashMap::from([
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine"),
    ]);

    let sum = input
        .lines()
        .map(|l| {
            let mut ll = String::from(l);
            for (key, value) in &number_map {
                ll = ll.replace(key, value);
            }
            ll
        })
        .map(|l| {
            let mut it =
                l.chars().filter_map(|c| c.to_digit(10));
            let first = it.next().expect(
                "There should be a number on each line",
            );
            let last = it.last().unwrap_or(first);

            format!("{first}{last}").parse::<u32>().expect(
                "First and last number should be parsable",
            )
        })
        .sum();
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(281, process(input)?);
        Ok(())
    }
}
