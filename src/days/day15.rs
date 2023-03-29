use rayon::prelude::*;
use regex::Regex;
use std::{
    cmp::Ordering,
    io::{BufRead, Lines},
};

#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    bx: i32,
    by: i32,
    manhattan: usize,
}

impl Sensor {
    fn new(x: i32, y: i32, bx: i32, by: i32) -> Self {
        let manhattan = (i32::abs(bx - x) + i32::abs(by - y)) as usize;
        Sensor {
            x,
            y,
            bx,
            by,
            manhattan,
        }
    }

    fn intercept(&self, row: i32, count_beacon: bool) -> Option<(i32, i32)> {
        let distance = i32::abs(row - self.y);
        let rest = self.manhattan as i32 - distance;

        if rest < 0 {
            None
        } else if !count_beacon && self.by == row {
            match self.bx.cmp(&self.x) {
                Ordering::Greater => Some((self.x - rest, self.x + rest - 1)),
                Ordering::Less => Some((self.x - rest + 1, self.x + rest)),
                Ordering::Equal => None,
            }
        } else {
            Some((self.x - rest, self.x + rest))
        }
    }
}

fn intersection(a: &(i32, i32), b: &(i32, i32)) -> Option<(i32, i32)> {
    let size_a = a.1 - a.0 + 1;
    let size_b = b.1 - b.0 + 1;
    let min = i32::min(a.0, b.0);
    let max = i32::max(a.1, b.1);

    if max - min + 1 > size_a + size_b {
        None
    } else {
        Some((min, max))
    }
}

#[test]
fn check_sensor() {
    let s = Sensor::new(8, 7, 2, 10);

    assert_eq!(s.intercept(-4, false), None);
    assert_eq!(s.intercept(-2, false), Some((8, 8)));
    assert_eq!(s.intercept(0, false), Some((6, 10)));
    assert_eq!(s.intercept(6, false), Some((0, 16)));
    assert_eq!(s.intercept(7, false), Some((-1, 17)));
    assert_eq!(s.intercept(10, false), Some((3, 14)));
    assert_eq!(s.intercept(10, true), Some((2, 14)));
    assert_eq!(s.intercept(17, false), None);

    assert_eq!(intersection(&(12, 12), &(2, 14)), Some((2, 14)));
    assert_eq!(intersection(&(2, 14), &(12, 12)), Some((2, 14)));
    assert_eq!(intersection(&(-2, 2), &(2, 4)), Some((-2, 4)));
    assert_eq!(intersection(&(2, 4), &(-2, 2)), Some((-2, 4)));
    assert_eq!(intersection(&(3, 5), &(-2, 2)), Some((-2, 5)));
    assert_eq!(intersection(&(4, 6), &(-2, 2)), None);
}

fn push_and_merge(list: &mut Vec<(i32, i32)>, range: &(i32, i32)) {
    for r in list.iter_mut() {
        let intersection = intersection(range, r);

        if let Some(intersection) = intersection {
            *r = intersection;
            return;
        }
    }
    list.push(*range);
}

fn merge_ranges(mut list: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    list.sort();

    list.iter().fold(vec![], |mut acc, r| {
        push_and_merge(&mut acc, r);
        acc
    })
}

fn ranges_on_row(
    sensors: &Vec<Sensor>,
    row: i32,
    count_beacon: bool,
    delim: Option<&(i32, i32)>,
) -> Vec<(i32, i32)> {
    let mut ranges = vec![];

    for s in sensors {
        let range = s.intercept(row, count_beacon);

        if let Some(mut range) = range {
            if let Some(delim) = delim {
                range = (i32::max(delim.0, range.0), i32::min(delim.1, range.1));
                if range.0 > range.1 {
                    continue;
                }
            }
            push_and_merge(&mut ranges, &range);
        }
    }

    merge_ranges(ranges)
}

fn resolve<T>(lines: Lines<T>) -> (i32, i64)
where
    T: BufRead,
{
    let beacon_regex = Regex::new(r"-?\d+").unwrap();
    let sensors = Vec::from_iter(lines.map(|line| {
        let line = line.unwrap();

        let mut beacon_values = beacon_regex
            .find_iter(&line)
            .map(|x| x.as_str().parse::<i32>().unwrap());

        let x = beacon_values.next().unwrap();
        let y = beacon_values.next().unwrap();
        let bx = beacon_values.next().unwrap();
        let by = beacon_values.next().unwrap();

        Sensor::new(x, y, bx, by)
    }));

    let row;
    let size;

    if cfg!(test) {
        row = 10;
        size = 20;
    } else {
        row = 2000000;
        size = 4000000;
    }

    let part2 = (0..size + 1)
        //.into_iter()
        //.find_map(|row| {
        .into_par_iter()
        .find_map_any(|row| {
            let ranges = ranges_on_row(&sensors, row, true, Some(&(0, size)));

            if ranges.len() == 2 {
                Some((ranges[0].1 as i64 + 1) * 4000000 + row as i64)
            } else {
                None
            }
        })
        .unwrap();

    (
        ranges_on_row(&sensors, row, false, None)
            .iter()
            .map(|(left, right)| right - left + 1)
            .sum(),
        part2,
    )
}

#[test]
fn check() {
    const TEST: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
    use std::io::Cursor;

    let (part1, part2) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, 26);
    assert_eq!(part2, 56000011);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2022::Day::new(file!(), resolve_string) }
