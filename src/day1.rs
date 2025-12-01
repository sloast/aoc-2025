use std::io::BufRead;

use nom::{
    Parser,
    branch::alt,
    character::{char, complete::digit1},
    combinator::{all_consuming, map_res, recognize, value},
    sequence::pair,
};

use crate::Input;

fn parse(s: &str) -> (i32, i32) {
    let res: Result<(&str, (i32, i32)), nom::Err<()>> = all_consuming(pair(
        alt((value(-1, char('L')), value(1, char('R')))),
        map_res(recognize(digit1), str::parse),
    ))
    .parse(s.as_ref());

    res.unwrap().1
}

pub fn part1(input: Input) -> String {
    let mut pos: i32 = 50;
    let mut total = 0;

    for line in input.lines() {
        let (dir, n) = parse(&line.unwrap());

        pos += dir * n;
        pos = pos.rem_euclid(100);
        if pos == 0 {
            total += 1;
        }
    }

    total.to_string()
}

pub fn part2(input: Input) -> String {
    let mut pos: i32 = 50;
    let mut total = 0;

    for line in input.lines() {
        let (dir, n) = parse(&line.unwrap());

        pos = pos.rem_euclid(100);
        if pos == 0 && dir == -1 {
            pos = 100;
        }
        pos += dir * n;
        if pos <= 0 {
            total += (100 - pos) / 100;
        } else {
            total += pos / 100;
        }
    }

    total.to_string()
}
