use std::{cmp::max, io::BufRead, str::FromStr};

use anyhow::Result;
use nom::{
    Parser,
    character::{char, complete::digit1},
    combinator::{all_consuming, complete, map_res, recognize},
    error::ErrorKind,
};

use crate::Input;

fn parse<T: FromStr>(s: &str) -> (T, T) {
    fn num_parser<'a, T: FromStr>() -> impl Parser<&'a str, Output = T, Error = (&'a str, ErrorKind)>
    {
        map_res(recognize(digit1), str::parse)
    }
    let mut parser = all_consuming(complete((num_parser(), char('-'), num_parser())));
    let (_, (a, _, b)) = parser.parse(s.as_ref()).unwrap();
    (a, b)
}

fn get_ranges(lines: &mut impl Iterator<Item = String>) -> Vec<(u64, u64)> {
    let mut ranges_raw: Vec<(u64, u64)> = vec![];
    for line in lines.by_ref().take_while(|l| !l.is_empty()) {
        ranges_raw.push(parse(&line));
    }
    ranges_raw.sort_by_key(|x| x.0);

    let mut ranges: Vec<(u64, u64)> = vec![];
    for x in ranges_raw {
        if let Some(y) = ranges.last_mut()
            && x.0 <= y.1
        {
            y.1 = max(x.1, y.1);
        } else {
            ranges.push(x);
        }
    }
    ranges
}

pub fn part1(input: Input) -> Result<usize> {
    let mut lines = input.lines().map(|x| x.unwrap());
    let ranges = get_ranges(&mut lines);

    let count = lines
        .map(|l| l.parse().unwrap())
        .filter(|&n| match ranges.get(ranges.partition_point(|x| x.1 < n)) {
            Some(&(a, _)) => a <= n,
            None => false,
        })
        .count();

    Ok(count)
}

pub fn part2(input: Input) -> Result<u64> {
    let mut lines = input.lines().map(|x| x.unwrap());
    let ranges = get_ranges(&mut lines);

    Ok(ranges.into_iter().map(|(a, b)| b - a + 1).sum())
}
