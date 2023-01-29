use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, Lines};

fn segments(point_a: (i32, i32), point_b: (i32, i32)) -> Vec<(i32, i32)> {
    let mut points = vec![];
    let v: (i32, i32) = (
        (point_b.0 - point_a.0).signum(),
        (point_b.1 - point_a.1).signum(),
    );
    let mut p = point_a;

    loop {
        points.push(p);

        if p == point_b {
            break;
        }

        p = (p.0 + v.0, p.1 + v.1);
    }

    points
}

#[allow(dead_code)]
fn print_hash(verticals: &HashMap<i32, Vec<i32>>, maxy: i32) {
    for y in 0..maxy + 2 {
        let mut row = String::from("");

        for x in 404..594 {
            let vertical = verticals.get(&x);

            if let Some(vertical) = vertical {
                if vertical.contains(&y) {
                    row.push('#');
                } else {
                    row.push('.');
                }
            } else {
                row.push('.');
            }
        }
        println!("{row}");
    }
}

fn get_floor(verticals: &mut HashMap<i32, Vec<i32>>, (x, y): (i32, i32)) -> Option<i32> {
    let vertical = verticals.get_mut(&x)?;

    vertical.iter().find(|&x| x >= &y).copied()
}

fn insert_sorted(sorted: &mut Vec<i32>, value: i32) {
    sorted.insert(sorted.binary_search(&value).unwrap_err(), value);
}

fn add_sand_part1(verticals: &mut HashMap<i32, Vec<i32>>, (x, y): (i32, i32)) -> Option<i32> {
    let mut x = x;
    let mut y = get_floor(verticals, (x, y))?;

    loop {
        let left = get_floor(verticals, (x - 1, y))?;

        if left == y {
            // can't go left, let's try right
            let right = get_floor(verticals, (x + 1, y))?;

            if right == y {
                // no left, no rigth, stay here
                let vertical = verticals.get_mut(&x).unwrap();

                insert_sorted(vertical, y - 1);

                return Some(y - 1);
            } else {
                x = x + 1;
                y = right;
            }
        } else {
            x = x - 1;
            y = left;
        }
    }
}

fn add_sand_part2(
    verticals: &mut HashMap<i32, Vec<i32>>,
    (x, y): (i32, i32),
    maxy: i32,
) -> Option<i32> {
    let mut x = x;
    let mut y = get_floor(verticals, (x, y)).unwrap_or(maxy);

    loop {
        let left = get_floor(verticals, (x - 1, y)).unwrap_or(maxy);

        if left == y {
            // can't go left, let's try right
            let right = get_floor(verticals, (x + 1, y)).unwrap_or(maxy);

            if right == y {
                // no left, no rigth, stay here
                let vertical = verticals.entry(x).or_insert(vec![]);

                insert_sorted(vertical, y - 1);

                return Some(y - 1);
            } else {
                x = x + 1;
                y = right;
            }
        } else {
            x = x - 1;
            y = left;
        }
    }
}

fn resolve<T>(lines: Lines<T>) -> (usize, u32)
where
    T: BufRead,
{
    let point_regex = Regex::new(r"\d+,\d+").unwrap();
    let mut rocks: HashSet<(i32, i32)> = HashSet::new();
    let mut maxy = 0;

    for line in lines {
        let line = line.unwrap();

        let points = point_regex
            .find_iter(&line)
            .map(|x| {
                let mut split = x.as_str().split(',');
                let x = split.next().unwrap().parse::<i32>().unwrap();
                let y = split.next().unwrap().parse::<i32>().unwrap();

                maxy = i32::max(maxy, y);
                (x, y)
            })
            .collect::<Vec<_>>();

        for i in 1..points.len() {
            for p in segments(points[i - 1], points[i]) {
                rocks.insert(p);
            }
        }
    }

    let mut verticals1: HashMap<i32, Vec<i32>> = HashMap::new();

    for (x, y) in rocks {
        let vertical = verticals1.entry(x).or_insert(vec![]);

        vertical.push(y);
    }

    verticals1.values_mut().for_each(|x| x.sort());

    let mut verticals2 = verticals1.clone();

    let mut part1 = 0;
    let mut part2 = 1;

    while add_sand_part1(&mut verticals1, (500, 0)).is_some() {
        part1 += 1;
    }

    while add_sand_part2(&mut verticals2, (500, 0), maxy + 2) != Some(0) {
        part2 += 1;
    }

    (part1, part2)
}

#[test]
fn check() {
    const TEST: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    use std::io::Cursor;

    let (part1, part2) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, 24);
    assert_eq!(part2, 93);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2022::Day::new(file!(), resolve_string) }
