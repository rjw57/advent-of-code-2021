use itertools::Itertools;
use std::{
    cmp::{max, min},
    fs::File,
    io::{self, BufRead},
    str::FromStr, iter::zip,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: FromStr> FromStr for Point<T> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y): (T, T) = match s.split(",").map(|v| v.parse()).collect_tuple() {
            Some((Ok(x), Ok(y))) => (x, y),
            _ => return Err("failed to parse point"),
        };
        return Ok(Point { x, y });
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Line<T> {
    start: Point<T>,
    end: Point<T>,
}

impl<T: FromStr> FromStr for Line<T> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end): (Point<T>, Point<T>) =
            match s.split(" -> ").map(|v| v.parse()).collect_tuple() {
                Some((Ok(s), Ok(e))) => (s, e),
                _ => return Err("failed to parse line"),
            };
        return Ok(Line { start, end });
    }
}

fn main() {
    let file = File::open("./input.txt").expect("could not open input");
    let lines: Vec<Line<usize>> = io::BufReader::new(file)
        .lines()
        .map(|v| v.expect("failed to read"))
        .map(|v| v.parse().expect("failed to parse"))
        .collect();

    let max_x = lines.iter().fold(0, |a, l| max(a, max(l.start.x, l.end.x)));
    let max_y = lines.iter().fold(0, |a, l| max(a, max(l.start.y, l.end.y)));

    let mut overlap_counts = vec![vec![0; max_y + 1]; max_x + 1];
    for l in &lines {
        if l.start.x == l.end.x {
            // vertical
            let sy = min(l.start.y, l.end.y);
            let ey = max(l.start.y, l.end.y);
            for y in sy..=ey {
                overlap_counts[l.start.x][y] += 1;
            }
        } else if l.start.y == l.end.y {
            // horizontal
            let sx = min(l.start.x, l.end.x);
            let ex = max(l.start.x, l.end.x);
            for x in sx..=ex {
                overlap_counts[x][l.start.y] += 1;
            }
        }
    }

    let mut more_2_count: usize = 0;
    for x in 0..=max_x {
        for y in 0..=max_y {
            if overlap_counts[x][y] >= 2 {
                more_2_count += 1;
            }
        }
    }

    println!("no diags: points with >=2 overlaps: {}", more_2_count);

    for l in &lines {
        let dx = max(l.start.x, l.end.x) - min(l.start.x, l.end.x);
        let dy = max(l.start.y, l.end.y) - min(l.start.y, l.end.y);
        if dx == dy {
            let xs: Vec<_> = if l.start.x < l.end.x {
                (l.start.x..=l.end.x).collect()
            } else {
                (l.end.x..=l.start.x).rev().collect()
            };
            let ys: Vec<_> = if l.start.y < l.end.y {
                (l.start.y..=l.end.y).collect()
            } else {
                (l.end.y..=l.start.y).rev().collect()
            };
            for (x, y) in zip(xs, ys) {
                overlap_counts[x][y] += 1;
            }
        }
    }

    more_2_count = 0;
    for x in 0..=max_x {
        for y in 0..=max_y {
            if overlap_counts[x][y] >= 2 {
                more_2_count += 1;
            }
        }
    }

    println!("with diags: points with >=2 overlaps: {}", more_2_count);
}
