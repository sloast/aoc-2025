use std::{io::read_to_string, str::FromStr};

use nom::{
    Parser,
    character::{char, complete::digit1},
    combinator::{all_consuming, complete, map_res, recognize},
    error::ErrorKind,
    multi::separated_list0,
};

use crate::Input;

fn parse<T: FromStr>(s: &str) -> Vec<(T, T)> {
    fn num_parser<'a, T: FromStr>() -> impl Parser<&'a str, Output = T, Error = (&'a str, ErrorKind)>
    {
        map_res(recognize(digit1), str::parse)
    }
    let mut parser = all_consuming(complete(separated_list0(
        char(','),
        (num_parser(), char('-'), num_parser()).map(|(a, _, b)| (a, b)),
    )));

    let (_, vec) = parser.parse(s.as_ref()).unwrap();

    vec
}

fn run(input: Input, only_2: bool) -> u64 {
    let ids = parse::<String>(read_to_string(input).unwrap().trim());

    ids.iter()
        .map(|(a, b)| [a.as_ref(), b.as_ref()])
        .flat_map(|[a, b]: [&str; 2]| {
            let mut ids: Vec<String> = vec![];

            for n in 2..=if only_2 { 2 } else { b.len() } {
                let (mut a1, mut a2) = (
                    a[0..a.len() / n].to_owned(),
                    a[a.len() / n..a.len()].to_owned(),
                );
                let (mut b1, mut b2) = (
                    b[0..b.len().div_ceil(n)].to_owned(),
                    b[b.len().div_ceil(n)..b.len()].to_owned(),
                );
                if a2.len() > b2.len() {
                    continue;
                }
                if a.len() % n != 0 {
                    a1 = "1".to_owned() + &"0".repeat(a.len().div_ceil(n) - 1);
                    a2 = a1.repeat(n - 1);
                }
                if b.len() % n != 0 {
                    b1 = "9".repeat(b.len() / n);
                    b2 = b1.repeat(n - 1);
                }

                macro_rules! push {
                    ($x:expr) => {
                        ids.push($x.to_string().repeat(n))
                    };
                }
                macro_rules! int {
                    ($x:expr) => {
                        $x.parse::<u64>().unwrap()
                    };
                }
                let [a1n, a2n, b1n, b2n] = [&a1, &a2, &b1, &b2].map(|x| int!(x));

                if int!(a1.repeat(n - 1)) >= a2n {
                    push!(a1)
                }
                if int!(b1.repeat(n - 1)) <= b2n {
                    push!(b1)
                }
                if a1n == b1n {
                    ids.pop();
                }
                for x in a1n + 1..b1n {
                    push!(x)
                }
            }
            ids.sort();
            ids.dedup();
            ids
        })
        .map(|x| x.parse::<u64>().unwrap())
        .sum()
}

pub fn part1(input: Input) -> u64 {
    run(input, true)
}

pub fn part2(input: Input) -> u64 {
    run(input, false)
}
