use crate::custom_error::AocError;
use std::collections::HashSet;

#[derive(Debug)]
struct Num {
    number: u32,
    begin: usize,
    end: usize,
    row: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Symbol(usize, usize);

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u32, AocError> {
    let mut nums: Vec<Num> = Vec::new();
    let mut cur: Vec<char> = Vec::new();

    let mut should_insert = false;
    let mut begin = 0;
    let mut end = 0;

    let symbols = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(j, char)| {
                    !char.is_ascii_digit() && *char != '.'
                })
                .flat_map(move |(j, _)| {
                    [
                        (i + 1, j),
                        (i - 1, j),
                        (i, j + 1),
                        (i, j - 1),
                        (i + 1, j + 1),
                        (i - 1, j + 1),
                        (i + 1, j - 1),
                        (i - 1, j - 1),
                    ]
                })
        })
        .map(|(i, j)| Symbol(i, j))
        .collect::<HashSet<Symbol>>();

    for (i, line) in input.lines().enumerate() {
        should_insert = false;
        for (j, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                if cur.is_empty() {
                    begin = j;
                }
                end = j;
                if symbols.contains(&Symbol(i, j)) {
                    should_insert = true;
                }

                cur.push(char)
            } else if !cur.is_empty() {
                let num = Num {
                    number: cur
                        .iter()
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap(),
                    begin,
                    end,
                    row: i,
                };
                if should_insert {
                    nums.push(num);
                }
                should_insert = false;
                cur.clear();
            }
        }
        if !cur.is_empty() {
            let num = Num {
                number: cur
                    .iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap(),
                begin,
                end,
                row: i,
            };
            if should_insert {
                nums.push(num);
            }
            should_insert = false;
            cur.clear();
        }
    }

    Ok(nums.iter().map(|num| num.number).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(4361, process(input)?);
        Ok(())
    }
}
