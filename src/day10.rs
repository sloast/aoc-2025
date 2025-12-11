use std::{io::BufRead, str::FromStr};

use anyhow::{Result, anyhow};
// use fxhash::FxHashSet;
// use indicatif::ProgressBar;
use nom::{
    Parser,
    character::{
        char,
        complete::{self, space1},
        streaming::one_of,
    },
    combinator::{all_consuming, complete, map_res, recognize},
    error::ErrorKind,
    multi::{many1, separated_list0},
    sequence::delimited,
};

use crate::Input;

fn parse<T: FromStr>(s: &str) -> (Vec<bool>, Vec<Vec<T>>, Vec<T>) {
    fn num_parser<'a, T: FromStr>() -> impl Parser<&'a str, Output = T, Error = (&'a str, ErrorKind)>
    {
        map_res(recognize(complete::digit1), str::parse)
    }
    let mut parser = all_consuming(complete((
        delimited(
            char('['),
            many1(one_of(".#").map(|c| match c {
                '.' => false,
                '#' => true,
                _ => panic!(),
            })),
            char(']'),
        ),
        delimited(
            space1,
            separated_list0(
                char(' '),
                delimited(
                    char('('),
                    separated_list0(char(','), num_parser::<T>()),
                    char(')'),
                ),
            ),
            space1,
        ),
        delimited(
            char('{'),
            separated_list0(char(','), num_parser::<T>()),
            char('}'),
        ),
    )));

    let (_, res) = parser.parse(s.as_ref()).unwrap();
    res
}

#[inline]
fn part1_press(lights: &mut [bool], button: &[usize]) {
    for &x in button {
        lights[x] = !lights[x]
    }
}

fn part1_search(target_lights: &[bool], buttons: &[Vec<usize>], depth: usize) -> Option<usize> {
    let mut stack: Vec<usize> = vec![0];
    let mut current_lights = vec![false; target_lights.len()];

    while let Some(&last) = stack.last() {
        if last == buttons.len() {
            stack.pop();
            if let Some(last) = stack.last_mut() {
                part1_press(&mut current_lights, &buttons[*last]);
                *last += 1;
            }
            continue;
        }
        part1_press(&mut current_lights, &buttons[last]);
        if stack.len() <= depth {
            stack.push(last + 1);
        } else if current_lights == target_lights {
            return Some(stack.len());
        } else {
            part1_press(&mut current_lights, &buttons[last]);
            *stack.last_mut().unwrap() += 1;
        }
    }

    None
}

pub fn part1(input: Input) -> Result<usize> {
    let mut total = 0;
    for line in input.lines() {
        let line = line?;
        let (lights, buttons, _) = parse::<usize>(line.as_ref());
        total += (0..buttons.len())
            .flat_map(|i| part1_search(&lights, &buttons, i))
            .next()
            .ok_or_else(|| anyhow!("no solution found for line: {}", line))?;
    }

    Ok(total)
}

// #[inline]
// fn part2_press(joltage: &mut [usize], button: &[usize]) {
//     for &x in button {
//         joltage[x] += 1
//     }
// }

// #[inline]
// fn part2_unpress(joltage: &mut [usize], button: &[usize]) {
//     for &x in button {
//         joltage[x] -= 1
//     }
// }

// fn part2_search(target_joltage: &[usize], buttons: &[Vec<usize>], depth: usize) -> Option<usize> {
//     let mut stack: Vec<usize> = vec![0];
//     let mut current_joltage = vec![0; target_joltage.len()];
//     let mut map: FxHashSet<Vec<usize>> = FxHashSet::default();

//     while let Some(&last) = stack.last() {
//         if last == buttons.len() {
//             stack.pop();
//             if let Some(last) = stack.last_mut() {
//                 part2_unpress(&mut current_joltage, &buttons[*last]);
//                 map.insert(current_joltage.clone());
//                 *last += 1;
//             }
//             continue;
//         }
//         part2_press(&mut current_joltage, &buttons[last]);
//         if current_joltage == target_joltage {
//             return Some(stack.len());
//         } else if stack.len()
//             + target_joltage
//                 .iter()
//                 .zip(current_joltage.iter())
//                 .map(|(&a, &b)| a.saturating_sub(b))
//                 .min()
//                 .unwrap()
//             <= depth
//             && target_joltage
//                 .iter()
//                 .zip(current_joltage.iter())
//                 .all(|(a, b)| a >= b)
//             && !map.contains(&current_joltage)
//             && target_joltage
//                 .iter()
//                 .zip(current_joltage.iter())
//                 .enumerate()
//                 .filter(|&(_, (a, b))| a > b)
//                 .all(|(i, _)| buttons[last..].iter().any(|button| button.contains(&i)))
//         {
//             stack.push(last);
//         } else {
//             map.insert(current_joltage.clone());
//             part2_unpress(&mut current_joltage, &buttons[last]);
//             *stack.last_mut().unwrap() += 1;
//         }
//     }

//     None
// }

// pub fn part2(input: Input) -> Result<usize> {
//     let mut total = 0;
//     let bar = ProgressBar::new(154);
//     for line in input.lines() {
//         let line = line?;
//         let (_, mut buttons, joltage) = parse::<usize>(line.as_ref());
//         buttons.sort_by_key(|x| -(x.len() as isize));
//         total += (*joltage.iter().min().unwrap()..)
//             .flat_map(|i| part2_search(&joltage, &buttons, i))
//             .next()
//             .ok_or_else(|| anyhow!("no solution found for line: {}", line))?;
//         bar.inc(1);
//     }
//     bar.finish_and_clear();

//     Ok(total)
// }

pub fn part2(_input: Input) -> Result<&'static str> {
    Ok("[todo]")
}
