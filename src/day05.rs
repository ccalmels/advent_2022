use regex::Regex;
use std::io::{BufRead, Lines};

fn resolve<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let move_regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let mut crates: Vec<(Vec<String>, Vec<String>)> = vec![];
    let mut lines = lines;

    for line in lines.by_ref() {
        let line = line.unwrap();

        if line == "" {
            break;
        }

        let len = (line.len() + 1) / 4;

        if crates.is_empty() {
            crates = vec![(vec![], vec![]); len];
        }

        for i in 0..len {
            let s = &line[i * 4 + 1..i * 4 + 2];

            if s != " " {
                crates[i].0.push(s.to_string());
                crates[i].1.push(s.to_string());
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
        let v: Vec<String> = crates[from - 1].0.drain(..count).rev().collect();

        crates[to - 1].0.splice(..0, v);

        // part 2
        let v: Vec<String> = crates[from - 1].1.drain(..count).collect();

        crates[to - 1].1.splice(..0, v);
    }

    crates
        .iter()
        .fold((String::from(""), String::from("")), |s, c| {
            (s.0 + &c.0[0], s.1 + &c.1[0])
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
