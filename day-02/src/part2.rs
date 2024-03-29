use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u32, AocError> {
    let h = input
        .lines()
        .map(|line| {
            let mut num = 0;
            let mut blue = 0;
            let mut green = 0;
            let mut red = 0;

            let it = line.split([',', ';', ' ']);
            for s in it {
                if let Ok(n) = s.parse::<u32>() {
                    num = n;
                }
                if s == "blue" && num > blue {
                    blue = num;
                }
                if s == "green" && num > green {
                    green = num;
                }
                if s == "red" && num > red {
                    red = num;
                }
            }
            blue * green * red
        })
        .sum();
    Ok(h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
            ";
        assert_eq!(2286, process(input)?);
        Ok(())
    }
}
