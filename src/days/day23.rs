use std::collections::{HashMap, HashSet};
use std::io::{BufRead, Lines};

#[derive(Debug, Clone)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    fn next(self: &Self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::E,
            Direction::E => Direction::N,
        }
    }

    fn positions(self: &Self) -> [usize; 3] {
        match self {
            Direction::N => [0, 1, 2],
            Direction::S => [5, 6, 7],
            Direction::W => [0, 3, 5],
            Direction::E => [2, 4, 7],
        }
    }

    fn update(self: &Self, p: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::N => (p.0, p.1 - 1),
            Direction::S => (p.0, p.1 + 1),
            Direction::W => (p.0 - 1, p.1),
            Direction::E => (p.0 + 1, p.1),
        }
    }
}

#[allow(dead_code)]
fn print(points: &HashSet<(i32, i32)>) {
    let mut min = *points.iter().next().unwrap();
    let mut max = min;

    for (x, y) in points {
        min.0 = i32::min(min.0, *x);
        min.1 = i32::min(min.1, *y);
        max.0 = i32::max(max.0, *x);
        max.1 = i32::max(max.1, *y);
    }

    println!("min: {min:?}");
    for y in min.1..max.1 + 1 {
        let mut line = String::from("");
        for x in min.0..max.0 + 1 {
            if points.contains(&(x, y)) {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        println!("{line}");
    }
}

fn get_neighbor(p: (i32, i32)) -> [(i32, i32); 8] {
    [
        (p.0 - 1, p.1 - 1),
        (p.0, p.1 - 1),
        (p.0 + 1, p.1 - 1),
        (p.0 - 1, p.1),
        (p.0 + 1, p.1),
        (p.0 - 1, p.1 + 1),
        (p.0, p.1 + 1),
        (p.0 + 1, p.1 + 1),
    ]
}

fn next_position(p: (i32, i32), points: &HashSet<(i32, i32)>, start: Direction) -> (i32, i32) {
    let mut direction = start;
    let oks = get_neighbor(p)
        .iter()
        .map(|p| !points.contains(p))
        .collect::<Vec<_>>();

    if oks != [true; 8] {
        for _ in 0..4 {
            let check_positions = direction.positions();

            if check_positions.iter().all(|&i| oks[i]) {
                return direction.update(p);
            }

            direction = direction.next();
        }
    }
    p
}

fn update_points(points: &mut HashSet<(i32, i32)>, start: Direction) -> bool {
    let mut new_points: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

    for p in points.iter() {
        let new_p = next_position(*p, &points, start.clone());

        if let Some(v) = new_points.get_mut(&new_p) {
            v.push(*p);
        } else {
            new_points.insert(new_p, vec![*p]);
        }
    }

    let mut someone_moved = false;

    for (new_pos, old_pos) in &new_points {
        if old_pos.len() == 1 && old_pos[0] != *new_pos {
            someone_moved = true;
            points.remove(&old_pos[0]);
            points.insert(*new_pos);
        }
    }

    someone_moved
}

fn part1(points: &HashSet<(i32, i32)>) -> i32 {
    let mut iter = points.iter();
    let mut min = *iter.next().unwrap();
    let mut max = min;
    let mut count = 1;

    for (x, y) in iter {
        min.0 = i32::min(min.0, *x);
        min.1 = i32::min(min.1, *y);
        max.0 = i32::max(max.0, *x);
        max.1 = i32::max(max.1, *y);
        count += 1;
    }

    (1 + max.1 - min.1) * (1 + max.0 - min.0) - count
}

fn resolve<T>(lines: Lines<T>) -> (i32, u32)
where
    T: BufRead,
{
    let mut points = HashSet::new();

    for (y, line) in lines.enumerate() {
        let line = line.unwrap();

        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                points.insert((x as i32, y as i32));
            }
        }
    }

    //print(&points);

    let mut direction = Direction::N;
    let mut count = 10;

    for _ in 0..count {
        update_points(&mut points, direction.clone());

        direction = direction.next();
    }
    //print(&points);

    let part1 = part1(&points);

    while update_points(&mut points, direction.clone()) {
        count += 1;

        direction = direction.next();
    }

    (part1, count + 1)
}

#[test]
fn check() {
    const TEST: &str = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";
    use std::io::Cursor;

    let (part1, part2) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, 110);
    assert_eq!(part2, 20);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2022::Day::new(file!(), resolve_string) }
