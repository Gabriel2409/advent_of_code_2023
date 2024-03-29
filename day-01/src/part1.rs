use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u32, AocError> {
    let sum = input
        .lines()
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
    // for l in _input.lines() {
    //     let mut it = l.chars().filter(|s| s.is_numeric());
    //     let first = it.next().unwrap();
    //     let last = it.last().unwrap_or(first);
    //     let cur = [first, last]
    //         .into_iter()
    //         .collect::<String>()
    //         .parse::<u32>()
    //         .unwrap();
    //
    //     sum += cur;
    // }
    //
    // Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2 
pqr3stu8vwx 
a1b2c3d4e5f
treb7uchet 
";
        assert_eq!(142, process(input)?);
        Ok(())
    }
}
