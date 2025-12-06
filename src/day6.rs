use std::{io::BufRead, iter::from_fn};

use anyhow::Result;

use crate::Input;

pub fn part1(input: Input) -> Result<u64> {
    let lines: Vec<_> = input.lines().map(|x| x.unwrap()).collect();
    let mut iter_iter = lines.iter().map(|l| l.split_whitespace());
    let mut num_iters: Vec<_> = iter_iter.by_ref().take(lines.len() - 1).collect();
    let ops = iter_iter.next().unwrap();

    Ok(ops
        .map(|op_symbol| {
            let op = match op_symbol {
                "+" => |x, y| x + y,
                "*" => |x, y| x * y,
                _ => panic!(),
            };

            num_iters
                .iter_mut()
                .flat_map(|i| i.next())
                .map(|s| s.parse::<u64>().unwrap())
                .reduce(op)
                .unwrap()
        })
        .sum())
}

pub fn part2(input: Input) -> Result<u64> {
    let lines: Vec<_> = input.lines().map(|x| x.unwrap()).collect();
    let mut num_iters: Vec<_> = lines[0..lines.len() - 1]
        .iter()
        .map(|l| l.chars())
        .collect();
    let ops = lines.last().unwrap().split_whitespace();

    Ok(ops
        .map(|op_symbol| {
            let op = match op_symbol {
                "+" => |x, y| x + y,
                "*" => |x, y| x * y,
                _ => panic!(),
            };

            from_fn(|| {
                let s = num_iters
                    .iter_mut()
                    .flat_map(|i| i.next())
                    .collect::<String>();
                let s = s.trim();
                if s.is_empty() {
                    None
                } else {
                    Some(s.parse::<u64>().unwrap())
                }
            })
            .reduce(op)
            .unwrap()
        })
        .sum())
}
