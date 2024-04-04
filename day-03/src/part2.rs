use crate::custom_error::AocError;
use itertools::Itertools;

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

    let sum: u32 = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '*')
                .map(move |(j, c)| (i, j, c))
        })
        .map(|(row_no, col_no, _)| {
            let adj = tuple_nums
                .iter()
                .filter(|(i, begin_col, end_col, _)| {
                    *i + 1 >= row_no
                        && *i <= row_no + 1
                        && *begin_col <= col_no + 1
                        && *end_col + 1 >= col_no
                })
                .map(|(_, _, _, value)| *value)
                .collect::<Vec<_>>();

            if adj.len() == 2 {
                adj.iter().product()
            } else {
                0
            }
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
        assert_eq!(467835, process(input)?);
        Ok(())
    }
}
