use regex::Regex;
use std::io::{BufRead, Lines};

#[derive(Clone)]
struct Crate {
    part1: Vec<char>,
    part2: Vec<char>,
}

impl Crate {
    fn new() -> Self {
        Crate {
            part1: vec![],
            part2: vec![],
        }
    }
}

fn resolve<T>(mut lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let move_regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let mut crates: Vec<Crate> = vec![];

    for line in lines.by_ref() {
        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        let len = (line.len() + 1) / 4;

        if crates.is_empty() {
            crates = vec![Crate::new(); len];
        }

        for i in 0..len {
            let s = &line[i * 4 + 1..i * 4 + 2];

            if s != " " {
                crates[i].part1.push(s.chars().next().unwrap());
                crates[i].part2.push(s.chars().next().unwrap());
            }
        }
    }

    for line in lines {
        let line = line.unwrap();
        let move_capture = move_regex.captures(&line).unwrap();
        let count = move_capture
            .get(1)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let from = move_capture
            .get(2)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let to = move_capture
            .get(3)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        // part 1
        let v: Vec<char> = crates[from - 1].part1.drain(..count).rev().collect();

        crates[to - 1].part1.splice(..0, v);

        // part 2
        let v: Vec<char> = crates[from - 1].part2.drain(..count).collect();

        crates[to - 1].part2.splice(..0, v);
    }

    crates
        .iter()
        .fold((String::from(""), String::from("")), |mut s, c| {
            s.0.push(c.part1[0]);
            s.1.push(c.part2[0]);
            s
        })
}

#[test]
fn check() {
    const TEST: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    use std::io::Cursor;

    let (part1, part2) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, "CMZ");
    assert_eq!(part2, "MCD");
}

inventory::submit! { advent_2022::Day::new(file!(), resolve) }
