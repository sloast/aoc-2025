use std::{cell::Cell, collections::HashMap, io::BufRead, ops::Add};

use anyhow::Result;
use fxhash::FxHashMap;
use nom::{
    Parser,
    bytes::tag,
    character::complete::{alpha1, space1},
    combinator::{all_consuming, complete},
    multi::separated_list0,
};

use crate::Input;

#[derive(Debug, Clone, Copy)]
enum Output {
    Dev(usize),
    Out,
}

#[derive(Debug, Clone, Copy, Default)]
struct Counts {
    neither: usize,
    dac: usize,
    fft: usize,
    both: usize,
}

impl Counts {
    fn total(&self) -> usize {
        self.neither + self.dac + self.fft + self.both
    }

    fn to_arr(self) -> [usize; 4] {
        [self.neither, self.dac, self.fft, self.both]
    }
}

impl Add for Counts {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self.to_arr();
        let rhs = rhs.to_arr();
        for i in 0..4 {
            res[i] += rhs[i];
        }
        res.into()
    }
}

impl From<[usize; 4]> for Counts {
    fn from(arr: [usize; 4]) -> Self {
        Self {
            neither: arr[0],
            dac: arr[1],
            fft: arr[2],
            both: arr[3],
        }
    }
}

#[derive(Debug)]
struct Device {
    _name: String,
    con: Vec<Output>,
    n: Cell<Option<Counts>>,
}

fn parse_line(s: String) -> (String, Vec<String>) {
    let (_, (dev, _, con)) = all_consuming(complete((
        alpha1::<&str, ()>,
        tag(": "),
        separated_list0(space1, alpha1),
    )))
    .parse(s.as_ref())
    .unwrap();
    (dev.to_owned(), con.into_iter().map(str::to_owned).collect())
}

fn parse(input: Input, start: &str) -> (Vec<Device>, usize) {
    let mut map: FxHashMap<String, usize> = HashMap::default();
    let lines: Vec<_> = input.lines().map(Result::unwrap).map(parse_line).collect();
    for (i, (dev, _)) in lines.iter().enumerate() {
        map.insert(dev.to_owned(), i);
    }
    (
        lines
            .into_iter()
            .map(|(name, con)| Device {
                _name: name,
                con: con
                    .into_iter()
                    .map(|s| match s.as_ref() {
                        "out" => Output::Out,
                        _ => Output::Dev(*map.get(&s).unwrap()),
                    })
                    .collect(),
                n: None.into(),
            })
            .collect(),
        *map.get(start).unwrap(),
    )
}

fn search(devs: &[Device], start: usize) -> Counts {
    let dev = &devs[start];
    if let Some(n) = dev.n.get() {
        return n;
    }

    let mut res = dev
        .con
        .iter()
        .map(|&o| match o {
            Output::Dev(x) => search(devs, x),
            Output::Out => Counts {
                neither: 1,
                dac: 0,
                fft: 0,
                both: 0,
            },
        })
        .reduce(Add::add)
        .unwrap();

    if dev._name == "dac" {
        res.dac += res.neither;
        res.both += res.fft;
        res.neither = 0;
        res.fft = 0;
    } else if dev._name == "fft" {
        res.fft += res.neither;
        res.both += res.dac;
        res.neither = 0;
        res.dac = 0;
    }
    dev.n.set(Some(res));
    res
}

pub fn part1(input: Input) -> Result<usize> {
    let (devs, start) = parse(input, "you");
    Ok(search(&devs, start).total())
}

pub fn part2(input: Input) -> Result<usize> {
    let (devs, start) = parse(input, "svr");
    Ok(search(&devs, start).both)
}
