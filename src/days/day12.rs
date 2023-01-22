use std::collections::VecDeque;
use std::io::{BufRead, Lines};
use std::{thread, time::Duration};

fn neighbours(
    position: (usize, usize),
    grid: &Vec<Vec<u8>>,
    compare: fn(u8, u8) -> bool,
) -> Vec<(usize, usize)> {
    let mut ret: Vec<(usize, usize)> = vec![];
    let (x, y) = position;
    let value = grid[y][x];

    if x > 0 {
        ret.push((x - 1, y));
    }
    if x < grid[0].len() - 1 {
        ret.push((x + 1, y));
    }
    if y > 0 {
        ret.push((x, y - 1));
    }
    if y < grid.len() - 1 {
        ret.push((x, y + 1));
    }

    ret.into_iter()
        .filter(|&(ox, oy)| compare(grid[oy][ox], value))
        .collect()
}

#[allow(dead_code)]
fn print_bfs(lengths: &Vec<Vec<usize>>) {
    let h = lengths.len();
    let w = lengths[0].len();

    for y in 0..h {
        let mut row = String::from("");
        let mut sep = String::from("");

        for x in 0..w {
            let l = lengths[y][x];
            if l == h * w {
                row.push_str("   |");
            } else {
                let s = format!("{l:3}|");
                row.push_str(&s);
            }
            sep.push_str("---+");
        }
        println!("{row}");
        println!("{sep}");
    }
    thread::sleep(Duration::from_millis(1));
}

fn bfs<F>(
    grid: &Vec<Vec<u8>>,
    start: (usize, usize),
    finished: F,
    compare: fn(u8, u8) -> bool,
) -> usize
where
    F: Fn((usize, usize)) -> bool,
{
    let h = grid.len();
    let w = grid[0].len();
    let mut queue = VecDeque::new();
    let mut lengths = vec![vec![w * h; w]; h];

    lengths[start.1][start.0] = 0;

    queue.push_back(start);

    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        let length = lengths[pos.1][pos.0];

        if finished(pos) {
            return length;
        }

        for p in neighbours(pos, &grid, compare).iter() {
            let new_l = length + 1;
            let l = lengths[p.1][p.0];

            if new_l < l {
                lengths[p.1][p.0] = new_l;

                queue.push_back(*p);
            }
        }
        // print_bfs(&lengths);
    }
    0
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut grid = vec![];
    let mut start = (0, 0);
    let mut exit = (0, 0);

    for line in lines {
        let line = line.unwrap();
        let mut row = vec![];

        for (i, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (i, grid.len());
                row.push(0);
            } else if c == 'E' {
                exit = (i, grid.len());
                row.push(25);
            } else {
                row.push(c as u8 - 'a' as u8);
            }
        }
        grid.push(row);
    }

    let part1 = bfs(&grid, start, |pos| pos == exit, |a, b| a <= b + 1);
    let part2 = bfs(&grid, exit, |(x, y)| grid[y][x] == 0, |a, b| a + 1 >= b);

    (part1, part2)
}

#[test]
fn check() {
    const TEST: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    use std::io::Cursor;

    let (part1, part2) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, 31);
    assert_eq!(part2, 29);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2022::Day::new(file!(), resolve_string) }
