use std::io::BufRead;

use anyhow::Result;

use crate::Input;

fn parse(input: Input) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|s| {
            let s = s.unwrap();
            let mut iter = s.split(",").map(|e| e.parse::<i64>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .collect()
}

pub fn part1(input: Input) -> Result<i64> {
    let tiles = parse(input);

    Ok(tiles
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(i, a)| tiles[i..].iter().copied().map(move |b| (a, b)))
        .map(|(a, b)| ((1 + a.0 - b.0) * (1 + a.1 - b.1)).abs())
        .max()
        .unwrap())
}

fn limits((x, y): (i64, i64), tiles: &[(i64, i64)]) -> ((i64, i64), (i64, i64)) {
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (0, 0, i64::MAX, i64::MAX);
    for s in tiles.windows(4) {
        let [(sx0, sy0), (sx1, sy1), (sx2, sy2), (sx3, sy3)] = s.try_into().unwrap();

        if sy1 == sy2 {
            // Horizontal
            if sx1.min(sx2) <= x && sx1.max(sx2) >= x {
                if sx1 < sx2 {
                    if sy1 <= y && !(x == sx1 && sy0 < sy1 || x == sx2 && sy3 < sy2) {
                        min_y = min_y.max(sy1);
                    }
                } else if sy1 >= y && !(x == sx1 && sy0 > sy1 || x == sx2 && sy3 > sy2) {
                    max_y = max_y.min(sy1);
                }
            }
        }

        if sx1 == sx2 {
            // Vertical
            if sy1.min(sy2) <= y && sy1.max(sy2) >= y {
                if sy1 < sy2 {
                    if sx1 >= x && !(y == sy1 && sx0 > sx1 || y == sy2 && sx3 > sx2) {
                        max_x = max_x.min(sx1);
                    }
                } else if sx1 <= x && !(y == sy1 && sx0 < sx1 || y == sy2 && sx3 < sx2) {
                    min_x = min_x.max(sx1);
                }
            }
        }
    }

    ((min_x, min_y), (max_x, max_y))
}

pub fn part2(input: Input) -> Result<i64> {
    let tiles = parse(input);

    let tiles_wrapped: Vec<_> = tiles
        .iter()
        .chain(&tiles[tiles.len() - 3..tiles.len()])
        .cloned()
        .collect();
    let tiles_limits: Vec<_> = tiles
        .iter()
        .map(|&x| (x, limits(x, &tiles_wrapped)))
        .collect();

    Ok(tiles_limits
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(i, a)| tiles_limits[i..].iter().copied().map(move |b| (a, b)))
        .filter_map(|((a, (amin, amax)), (b, (bmin, bmax)))| {
            if (b.0 >= amin.0 && b.0 <= amax.0 && b.1 >= amin.1 && b.1 <= amax.1)
                && (a.0 >= bmin.0 && a.0 <= bmax.0 && a.1 >= bmin.1 && a.1 <= bmax.1)
            {
                Some((1 + (a.0 - b.0).abs()) * (1 + (a.1 - b.1).abs()))
            } else {
                None
            }
        })
        .max()
        .unwrap())
}
