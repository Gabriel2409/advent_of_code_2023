use std::collections::{
    btree_set::Intersection, HashMap, HashSet,
};

use nom::{
    bytes::complete::tag,
    character::complete::{
        alpha1, alphanumeric1, digit1, line_ending,
        multispace1, space1,
    },
    multi::separated_list1,
    sequence::{
        delimited, separated_pair, terminated, tuple,
    },
    IResult,
};

use crate::custom_error::AocError;

fn parse_directions(line: &str) -> IResult<&str, &str> {
    terminated(alpha1, multispace1)(line)
}

fn parse_row(
    line: &str,
) -> IResult<&str, (&str, (&str, &str))> {
    tuple((
        terminated(
            alphanumeric1,
            tuple((space1, tag("="), space1, tag("("))),
        ),
        tuple((
            terminated(
                alphanumeric1,
                tuple((tag(","), space1)),
            ),
            terminated(alphanumeric1, tag(")")),
        )),
    ))(line)
}

fn parse_rows(
    input: &str,
) -> IResult<&str, Vec<(&str, (&str, &str))>> {
    separated_list1(line_ending, parse_row)(input)
}

/// not working but you can get the LCM of all individual solutions.
/// It only works because the aoe input is made such that the cycle is a multiple
/// of the input sequence
#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u32, AocError> {
    let (input, directions) =
        parse_directions(input).unwrap();

    let (_, rows) = parse_rows(input).unwrap();

    let h = HashMap::<&str, (&str, &str)>::from_iter(rows);

    let (mut keys, final_keys): (
        HashSet<&str>,
        HashSet<&str>,
    ) = h
        .keys()
        .cloned()
        .filter(|key| {
            key.ends_with('A') || key.ends_with('Z')
        })
        .partition(|key| key.ends_with('A'));

    let mut cycle_hash = HashMap::new();
    let mut z_pos = HashMap::<&str, HashSet<usize>>::new();
    for initial_key in keys {
        let mut key = initial_key;
        let mut prev_conf = HashSet::new();
        directions.chars().enumerate().cycle().enumerate()
        .take_while(|(j, (i,c))| {

            if prev_conf.contains(&(*i,key))  {

                cycle_hash.insert(initial_key, (*i,*j));
                false
            } else {


                if key.ends_with('Z'){
                        z_pos.entry(initial_key).and_modify(|x| {x.insert(*j);}).or_insert(HashSet::from([*j]));
                    }

                prev_conf.insert((*i, key));
                match c {
                    'L' => {
                        key = h.get(key).unwrap().0;
                        true
                    }
                    'R' => {
                        key = h.get(key).unwrap().1;
                        true
                    }
                    _ => {
                        panic!("Should only contain L or R")
                    }
                }
            }
        })
        .map(|(i, _)| i + 1)
        .last()
        .unwrap();
    }
    dbg!(&cycle_hash);
    dbg!(&z_pos);

    let mut expanded_z_pos =
        HashMap::<&str, HashSet<u128>>::new();

    for i in 0..10000 {
        for (key, h_val) in z_pos.clone() {
            let cycle_size = cycle_hash.get(key).unwrap().1
                - cycle_hash.get(key).unwrap().0;

            for val in h_val {
                let new_val =
                    val as u128 + cycle_size as u128 * i;
                expanded_z_pos
                    .entry(key)
                    .and_modify(|x| {
                        x.insert(new_val);
                    })
                    .or_insert(HashSet::from([new_val]));
            }
        }
    }

    let mut z_val_iter = expanded_z_pos.values();

    let mut inter = z_val_iter.next().unwrap().clone();

    for hs in z_val_iter {
        inter = inter.intersection(hs).copied().collect();
    }

    let min = inter.iter().min().unwrap();
    Ok(*min as u32)

    // let nb = directions
    //     .chars()
    //     .cycle()
    //     .enumerate()
    //     .take_while(|(_, c)| {
    //         if keys.is_subset(&final_keys) {
    //             false
    //         } else {
    //             match c {
    //                 'L' => {
    //                     keys = keys
    //                         .iter()
    //                         .map(|key| {
    //                             h.get(key).unwrap().0
    //                         })
    //                         .collect();
    //
    //                     true
    //                 }
    //                 'R' => {
    //                     keys = keys
    //                         .iter()
    //                         .map(|key| {
    //                             h.get(key).unwrap().1
    //                         })
    //                         .collect();
    //                     true
    //                 }
    //                 _ => {
    //                     panic!("Should only contain L or R")
    //                 }
    //             }
    //         }
    //     })
    //     .map(|(i, _)| i + 1)
    //     .last()
    //     .unwrap();
    // Ok(nb as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(6 + 5, process(input)?);
        Ok(())
    }
}
