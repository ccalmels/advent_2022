use regex::Regex;
use std::io::{BufRead, Lines};

fn pop_and_add(stack: &mut Vec<usize>) -> usize {
    let current = stack.pop().unwrap();
    let last = stack.last_mut();

    if let Some(last) = last {
        *last += current;
    }
    current
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let command_regexp = Regex::new(r"^\$ (ls|cd \S+)$").unwrap();
    let size_regexp = Regex::new(r"^(\S+) \S+$").unwrap();
    let mut stack = vec![];
    let mut dirs = vec![];

    for line in lines {
        let line = line.unwrap();
        let command_capture = command_regexp.captures(&line);

        if let Some(command_capture) = command_capture {
            let command = command_capture.get(1).unwrap().as_str();

            match command {
                "ls" => (),
                "cd .." => dirs.push(pop_and_add(&mut stack)),
                _ => stack.push(0),
            };
        } else {
            let size_capture = size_regexp.captures(&line).unwrap();
            let size = size_capture.get(1).unwrap().as_str().parse::<usize>();

            if let Ok(size) = size {
                let last = stack.last_mut().unwrap();

                *last += size;
            }
        }
    }

    while !stack.is_empty() {
        dirs.push(pop_and_add(&mut stack));
    }

    dirs.sort();

    let needed = dirs.last().unwrap() - 40000000;

    (
        dirs.iter().filter(|x| x < &&100000).sum(),
        *dirs.iter().filter(|x| x > &&needed).next().unwrap(),
    )
}

#[test]
fn check() {
    const TEST: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    use std::io::Cursor;

    let (part1, part2) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, 95437);
    assert_eq!(part2, 24933642);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2022::Day::new(file!(), resolve_string) }
