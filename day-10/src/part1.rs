use crate::custom_error::AocError;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn find_starting_point(
    matrix: &[Vec<char>],
) -> (usize, usize) {
    for (i, row) in matrix.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == 'S' {
                return (i, j);
            }
        }
    }
    panic!("No S in matrix");
}

fn find_adj_pipes(
    matrix: &[Vec<char>],
    i: usize,
    j: usize,
) -> ((usize, usize), (usize, usize)) {
    let mut adj_pipes = Vec::new();

    if i > 0 {
        let c = matrix[i - 1][j];
        if c == '|' || c == '7' || c == 'F' {
            adj_pipes.push((i - 1, j));
        }
    }
    if j > 0 {
        let c = matrix[i][j - 1];
        if c == '-' || c == 'L' || c == 'F' {
            adj_pipes.push((i, j - 1));
        }
    }

    if i < matrix.len() - 1 {
        let c = matrix[i + 1][j];
        if c == '|' || c == 'L' || c == 'J' {
            adj_pipes.push((i + 1, j));
        }
    }
    if j < matrix[0].len() - 1 {
        let c = matrix[i][j + 1];
        if c == '-' || c == '7' || c == 'J' {
            adj_pipes.push((i, j + 1));
        }
    }

    if adj_pipes.len() != 2 {
        panic!("There should be 2 adj pipes")
    }
    (adj_pipes[0], adj_pipes[1])
}

fn find_next_pipe(
    prev: (usize, usize),
    cur: (usize, usize),
    matrix: &[Vec<char>],
) -> (usize, usize) {
    let (i, j) = cur;
    let character = matrix[i][j];
    dbg!((prev, cur, character));
    let possible_pos = match character {
        '-' => ((i, j - 1), (i, j + 1)),
        '|' => ((i - 1, j), (i + 1, j)),
        'F' => ((i + 1, j), (i, j + 1)),
        'L' => ((i - 1, j), (i, j + 1)),
        'J' => ((i - 1, j), (i, j - 1)),
        '7' => ((i + 1, j), (i, j - 1)),
        _ => panic!("Pb in pipes"),
    };

    if possible_pos.0 == prev {
        possible_pos.1
    } else {
        possible_pos.0
    }
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u32, AocError> {
    let matrix = parse_input(input);
    let (i, j) = find_starting_point(&matrix);

    let (mut p1, mut p2) = find_adj_pipes(&matrix, i, j);
    let mut prevp1 = (i, j);
    let mut prevp2 = (i, j);

    let mut c = 1;

    let count = loop {
        let nextp1 = find_next_pipe(prevp1, p1, &matrix);
        if nextp1 == p2 {
            break c;
        }
        (prevp1, p1) = (p1, nextp1);

        let nextp2 = find_next_pipe(prevp2, p2, &matrix);
        if nextp2 == p1 {
            break c + 1;
        }
        (prevp2, p2) = (p2, nextp2);

        c += 1;
    };

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(8, process(input)?);
        Ok(())
    }
}
