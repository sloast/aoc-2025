use std::{cell::Cell, io::BufRead, ops::Sub};

use anyhow::{Result, anyhow, bail};
use fxhash::FxHashMap;
use indicatif::ProgressBar;
use rayon::iter::{ParallelBridge, ParallelIterator};
use seq_macro::seq;

use crate::Input;

const BOXEL_SIDE: usize = 10_000;
const N_BOXELS: usize = 10;
const BUF_SIZE: usize = N_BOXELS * N_BOXELS * N_BOXELS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Pos(i64, i64, i64);

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct JunctionBox {
    pos: Pos,
    connections: Vec<Pos>,
}

impl Sub for Pos {
    type Output = i64;

    fn sub(self, rhs: Self) -> Self::Output {
        (rhs.0 - self.0).pow(2) + (rhs.1 - self.1).pow(2) + (rhs.2 - self.2).pow(2)
    }
}

#[derive(Debug)]
struct UFNode<'a> {
    pos: Pos,
    size: Cell<usize>,
    depth: Cell<usize>,
    parent: Cell<Option<&'a UFNode<'a>>>,
}

impl<'a> UFNode<'a> {
    fn root<'b>(&'b self) -> &'b UFNode<'a>
    where
        'a: 'b,
    {
        let mut curr = self;
        loop {
            match curr.parent.get() {
                None => return curr,
                Some(p) => curr = p,
            }
        }
    }

    fn union<'b>(&'a self, other: &'b Self)
    where
        'b: 'a,
    {
        if self.root().pos == other.root().pos {
            return;
        }
        let ra = self.root();
        let rb = other.root();
        let (a, b) = if ra.depth < rb.depth {
            (ra, rb)
        } else {
            (rb, ra)
        };
        b.size.set(a.size.get() + b.size.get());
        b.depth.set(b.depth.get().max(a.depth.get() + 1));
        a.parent.set(Some(b));
    }
}

fn uf_init<'a>(grid: &Grid) -> FxHashMap<Pos, UFNode<'a>> {
    let mut nodes = FxHashMap::default();
    for jb in grid.iter() {
        nodes.insert(
            jb.pos,
            UFNode {
                pos: jb.pos,
                size: 1.into(),
                depth: 0.into(),
                parent: None.into(),
            },
        );
    }
    nodes
}

fn union_find(grid: &Grid) -> Vec<usize> {
    let nodes = uf_init(grid);
    for jb in grid.iter() {
        let node = &nodes[&jb.pos];
        for other in jb.connections.iter() {
            node.union(&nodes[other]);
        }
    }

    let mut res: Vec<usize> = nodes
        .values()
        .filter_map(|e| {
            if e.parent.get().is_none() {
                Some(e.size.get())
            } else {
                None
            }
        })
        .collect();

    res.sort_unstable_by_key(|x| -(*x as i64));
    res
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<JunctionBox>>,
}

impl TryFrom<Input> for Grid {
    type Error = anyhow::Error;

    fn try_from(input: Input) -> std::result::Result<Self, Self::Error> {
        let mut data: Vec<Vec<_>> = vec![Vec::new(); BUF_SIZE];
        for line in input.lines() {
            let line = line?;
            let mut iter = line.split(",");
            macro_rules! next {
                () => {
                    iter.next()
                        .ok_or_else(|| anyhow!("not enough fields on line '{}'", line))?
                        .parse()?
                };
            }
            let pos: Pos = Pos(next!(), next!(), next!());
            let inx = Self::boxel_index(pos);
            data[inx].push(JunctionBox {
                pos,
                ..Default::default()
            });
        }
        Ok(Grid { data })
    }
}

impl Grid {
    fn iter(&self) -> impl Iterator<Item = &JunctionBox> {
        self.data.iter().flat_map(|b| b.iter())
    }

    fn boxel_pos(Pos(x, y, z): Pos) -> Pos {
        Pos(
            x / BOXEL_SIDE as i64 * BOXEL_SIDE as i64,
            y / BOXEL_SIDE as i64 * BOXEL_SIDE as i64,
            z / BOXEL_SIDE as i64 * BOXEL_SIDE as i64,
        )
    }

    fn boxel_index(Pos(x, y, z): Pos) -> usize {
        x as usize / BOXEL_SIDE
            + y as usize / BOXEL_SIDE * N_BOXELS
            + z as usize / BOXEL_SIDE * N_BOXELS * N_BOXELS
    }

    fn neighbour_boxels(pos: Pos) -> impl Iterator<Item = Pos> {
        let Pos(x, y, z) = Self::boxel_pos(pos);
        (-1..=1)
            .flat_map(|x: i64| (-1..=1).flat_map(move |y| (-1..=1).map(move |z| (x, y, z))))
            .filter(|&e| e != (0, 0, 0))
            .map(move |(dx, dy, dz)| {
                Pos(
                    x + dx * BOXEL_SIDE as i64,
                    y + dy * BOXEL_SIDE as i64,
                    z + dz * BOXEL_SIDE as i64,
                )
            })
            .filter(|&Pos(x, y, z)| {
                x.min(y).min(z) >= 0 && x.max(y).max(z) < (BOXEL_SIDE * N_BOXELS) as i64
            })
    }

    fn boxel_distance(pos: Pos, boxel: Pos) -> i64 {
        let mut dist = 0;
        seq!(I in 0..3 {
            if pos.I < boxel.I {
                dist += (pos.I - boxel.I).pow(2);
            } else if pos.I > boxel.I + BOXEL_SIDE as i64 {
                dist += (pos.I - (boxel.I + BOXEL_SIDE as i64)).pow(2);
            }
        });

        dist
    }

    fn closest_in_boxel(&self, jb: &JunctionBox, boxel: Pos) -> (Pos, i64) {
        Self::closest_from_iter(
            jb,
            self.data[Self::boxel_index(boxel)].iter().map(|jb| jb.pos),
        )
    }

    fn closest_from_iter(jb: &JunctionBox, iter: impl Iterator<Item = Pos>) -> (Pos, i64) {
        iter.filter(|x| *x != jb.pos && !jb.connections.contains(x))
            .map(|x| (x, jb.pos - x))
            .min_by_key(|(_, d)| *d)
            .unwrap_or((Pos::default(), i64::MAX))
    }

    fn closest(&self, jb: &JunctionBox) -> (Pos, i64) {
        let pos = jb.pos;
        let (mut closest, mut dist) = self.closest_in_boxel(jb, pos);

        for boxel in
            Self::neighbour_boxels(pos).filter(move |&b| Self::boxel_distance(pos, b) < dist)
        {
            let (new_closest, new_dist) = self.closest_in_boxel(jb, boxel);
            if new_dist < dist {
                (closest, dist) = (new_closest, new_dist);
            }
        }

        if dist == i64::MAX {
            (closest, dist) = Self::closest_from_iter(jb, self.iter().map(|jb| jb.pos));
        }

        (closest, dist)
    }

    fn get_mut(&mut self, pos: Pos) -> Option<&mut JunctionBox> {
        self.data[Self::boxel_index(pos)]
            .iter_mut()
            .find(|jb| jb.pos == pos)
    }
}

pub fn part1(input: Input) -> Result<usize> {
    let mut grid = Grid::try_from(input)?;

    let n = match grid.iter().count() {
        20 => 10,
        _ => 1000,
    };

    let bar = ProgressBar::new(n);

    for _ in 0..n {
        bar.inc(1);
        let (pos, (closest, _)) = grid
            .iter()
            .par_bridge()
            .map(|jb| (jb.pos, grid.closest(jb)))
            .min_by_key(|(_, (_, dist))| *dist)
            .ok_or_else(|| anyhow!("no connections found!"))?;
        if let Some(e) = grid.get_mut(closest) {
            e.connections.push(pos);
            grid.get_mut(pos).unwrap().connections.push(closest);
        } else {
            bail!("no more connections");
        }
    }

    bar.finish_and_clear();

    let circuit_sizes = union_find(&grid);

    Ok(circuit_sizes.iter().take(3).product())
}

pub fn part2(input: Input) -> Result<i64> {
    let mut grid = Grid::try_from(input)?;
    let n = grid.iter().count();

    let nodes = uf_init(&grid);

    let bar = ProgressBar::new(n as u64);

    let mut max_size = 0;

    let res = loop {
        bar.set_position(max_size as u64);
        let (pos, (closest, _)) = grid
            .iter()
            .par_bridge()
            .map(|jb| (jb.pos, grid.closest(jb)))
            .min_by_key(|(_, (_, dist))| *dist)
            .ok_or_else(|| anyhow!("no connections found!"))?;
        if let Some(e) = grid.get_mut(closest) {
            e.connections.push(pos);
            grid.get_mut(pos).unwrap().connections.push(closest);
            nodes[&pos].union(&nodes[&closest]);
            max_size = max_size.max(nodes[&pos].root().size.get());
            if max_size == n {
                break pos.0 * closest.0;
            }
        } else {
            bail!("no more connections")
        }
    };

    bar.finish_and_clear();

    Ok(res)
}
