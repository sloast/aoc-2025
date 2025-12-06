use std::{io::BufRead, mem};

use anyhow::{Result, anyhow, bail};

use crate::Input;

fn run(input: Input) -> Result<(usize, usize)> {
    let mut lines = input.lines();
    let mut upper = lines
        .next()
        .ok_or(anyhow!("input is empty"))??
        .chars()
        .map(|c| {
            Ok(match c {
                '.' => 0,
                'S' => 1,
                _ => bail!("invalid char: {}", c),
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let mut lower = vec![0; upper.len()];
    let mut splits = 0;

    for line in lines {
        for (i, c) in line?.chars().enumerate() {
            if upper[i] != 0 {
                match c {
                    '.' => lower[i] += upper[i],
                    '^' => {
                        lower[i - 1] += upper[i];
                        lower[i + 1] += upper[i];
                        splits += 1;
                    }
                    _ => bail!("invalid char: {}", c),
                }
            }
        }

        mem::swap(&mut upper, &mut lower);
        lower.fill(0);
    }

    Ok((splits, upper.iter().sum()))
}

pub fn part1(input: Input) -> Result<usize> {
    run(input).map(|(a, _)| a)
}

pub fn part2(input: Input) -> Result<usize> {
    run(input).map(|(_, b)| b)
}
