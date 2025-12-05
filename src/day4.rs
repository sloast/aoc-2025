use std::io::BufRead;

use anyhow::Result;

use crate::Input;

pub fn part1(input: Input) -> Result<usize> {
    let grid = PaperGrid::from(input);
    Ok(grid.count() - grid.remove_rolls().count())
}

pub fn part2(input: Input) -> Result<usize> {
    let mut grid = PaperGrid::from(input);
    let nb_rolls = grid.count();
    loop {
        let next = grid.remove_rolls();
        if next == grid {
            break;
        }
        grid = next;
    }
    Ok(nb_rolls - grid.count())
}

type Inner = u8;
#[derive(Debug, Clone, PartialEq, Eq)]
struct PaperGrid {
    data: Vec<Inner>,
    width: usize,
    height: usize,
}

impl PaperGrid {
    fn rows(&self) -> impl Iterator<Item = &[Inner]> {
        self.data.chunks(self.width)
    }
    fn get(&self, x: isize, y: isize) -> Inner {
        if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
            0
        } else {
            self.data[x as usize + y as usize * self.width]
        }
    }
    fn remove_rolls(&self) -> Self {
        Self {
            data: self
                .rows()
                .enumerate()
                .flat_map(|(y, row)| {
                    let grid1 = &self;
                    row.iter().enumerate().map(move |(x, &v)| {
                        if v == 0 {
                            return 0;
                        }
                        let can_remove = (-1..=1)
                            .flat_map(|dx| {
                                (-1..=1).map(move |dy| (dx + x as isize, dy + y as isize))
                            })
                            .filter(|&(x, y)| grid1.get(x, y) != 0)
                            .count()
                            < 5;
                        match can_remove {
                            true => 0,
                            false => 1,
                        }
                    })
                })
                .collect(),
            width: self.width,
            height: self.height,
        }
    }
    fn count(&self) -> usize {
        self.rows()
            .flat_map(|r| r.iter().filter(|&&x| x != 0))
            .count()
    }
}

impl From<Input> for PaperGrid {
    fn from(input: Input) -> Self {
        let mut width = 0;
        let vec: Vec<_> = input
            .lines()
            .flat_map(|l| {
                let l = l.unwrap();
                width = l.len();
                l.chars()
                    .map(|c| match c {
                        '.' => 0,
                        '@' => 1,
                        x => panic!("unexpected char {}", x),
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        Self {
            height: vec.len() / width,
            data: vec,
            width,
        }
    }
}
