use std::io::BufRead;

use crate::Input;

fn parse(input: Input) -> impl Iterator<Item = Vec<u64>> {
    input.lines().map(|x| {
        x.unwrap()
            .chars()
            .map(|c| c.to_digit(10).expect("failed to parse digit") as u64)
            .collect()
    })
}

fn process_bank<const N: usize>(bank: Vec<u64>) -> u64 {
    let mut digits = [0; N];
    let mut start = 0;

    for i in 0..N {
        (digits[i], start) = bank[0..bank.len() - (N - i - 1)]
            .iter()
            .copied()
            .zip(1..)
            .skip(start)
            .max_by(|(x1, i1), (x2, i2)| if x1 == x2 { i2.cmp(i1) } else { x1.cmp(x2) })
            .expect("bank is empty");
    }

    digits.iter().fold(0, |acc, x| acc * 10 + x)
}

pub fn part1(input: Input) -> u64 {
    parse(input).map(process_bank::<2>).sum()
}

pub fn part2(input: Input) -> u64 {
    parse(input).map(process_bank::<12>).sum()
}
