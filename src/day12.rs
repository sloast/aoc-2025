use std::{convert::identity, io::read_to_string};

use anyhow::Result;
use nom::{
    Parser,
    bytes::tag,
    character::{
        char,
        complete::{digit1, line_ending, one_of, space1},
    },
    combinator::all_consuming,
    multi::{many1, separated_list1},
};
use rayon::iter;

use crate::Input;

#[derive(Debug)]
struct Shape {
    index: usize,
    area: usize,
    data: Vec<Vec<bool>>,
}
type Region = ((usize, usize), Vec<usize>);

fn parse(input: Input) -> Result<(Vec<Shape>, Vec<Region>)> {
    let s = read_to_string(input)?.leak();

    let int = || digit1::<&str, (&str, nom::error::ErrorKind)>.map_res(str::parse::<usize>);

    let (_, res) = all_consuming((
        many1(
            (
                int(),
                char(':'),
                line_ending,
                separated_list1(
                    line_ending,
                    many1(one_of("#.").map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => panic!("invalid char '{}'", c),
                    })),
                ),
                (line_ending, line_ending),
            )
                .map(|(i, _, _, data, _)| Shape {
                    index: i,
                    area: data.iter().flatten().filter(|x| **x).count(),
                    data,
                }),
        ),
        many1(
            (
                int(),
                char('x'),
                int(),
                tag(": "),
                separated_list1(space1, int()),
                line_ending,
            )
                .map(|(x, _, y, _, vals, _)| ((x, y), vals)),
        ),
    ))
    .parse(s)?;

    Ok(res)
}

pub fn part1(input: Input) -> Result<i32> {
    let (shapes, regions) = parse(input)?;
    const SIZE: usize = 3;
    let mut yes = 0;
    for ((x, y), shape_counts) in regions {
        if (x / SIZE) * (y / SIZE) >= shape_counts.iter().sum() {
            yes += 1
        } else if x * y
            >= shape_counts
                .iter()
                .enumerate()
                .map(|(i, &n)| shapes[i].area * n)
                .sum()
        {
            return Ok(-1);
        }
    }
    Ok(yes)
}
