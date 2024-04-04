use crate::custom_error::AocError;
use itertools::Itertools;
use std::collections::HashSet;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u32, AocError> {
    let tuple_nums = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .group_by(|(_, c)| c.is_ascii_digit())
                .into_iter()
                .filter(|(cond, _)| *cond)
                .map(|(_, cols_and_group)| {
                    let mut begin_col = 0;
                    let mut end_col = 0;
                    let mut cur = String::new();
                    for (j, c) in cols_and_group {
                        if cur.is_empty() {
                            begin_col = j;
                        }
                        end_col = j;
                        cur.push(c);
                    }
                    (
                        i,
                        begin_col,
                        end_col,
                        cur.parse::<u32>().unwrap(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let symbols = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| {
                    !c.is_ascii_digit() && *c != '.'
                })
                .map(move |(j, c)| (i, j, c))
        })
        .collect::<HashSet<_>>();

    let mut extended = HashSet::new();
    for (i, j, _) in symbols {
        extended.extend([(i - 1, j - 1)]);
        extended.extend([(i - 1, j)]);
        extended.extend([(i - 1, j + 1)]);
        extended.extend([(i, j - 1)]);
        extended.extend([(i, j + 1)]);
        extended.extend([(i + 1, j - 1)]);
        extended.extend([(i + 1, j)]);
        extended.extend([(i + 1, j + 1)]);
    }

    let sum: u32 = tuple_nums
        .iter()
        .filter(|(i, begin_col, end_col, _)| {
            for j in *begin_col..=*end_col {
                if extended.contains(&(*i, j)) {
                    return true;
                }
            }
            false
        })
        .map(|(_, _, _, value)| value)
        .sum();

    Ok(sum)
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
