use std::io::{BufRead, Lines};

fn find_element(str1: &str, str2: &str) -> Option<char> {
    str1.chars().find(|c| str2.contains(*c))
}

fn element_priority(c: char) -> i32 {
    if c.is_lowercase() {
        c as i32 - 'a' as i32 + 1
    } else {
        c as i32 - 'A' as i32 + 27
    }
}

fn common_element(str1: &str, str2: &str, str3: &str) -> char {
    let mut ret = str1
        .chars()
        .filter(|c| str2.contains(*c))
        .filter(|c| str3.contains(*c))
        .collect::<Vec<_>>();

    ret.sort_unstable();
    ret.dedup();

    if ret.len() != 1 {
        panic!();
    }

    *ret.last().unwrap()
}

#[test]
fn check_common() {
    assert_eq!(common_element("AaB", "CaB", "DaE"), 'a');
    assert_eq!(common_element("AaBhX", "hCyBZ", "hDaEZ"), 'h');

    assert_eq!(
        common_element(
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg"
        ),
        'r'
    );

    assert_eq!(
        common_element(
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ),
        'Z'
    );
}

// more intuitive IMHO
fn _resolve<T>(lines: Lines<T>) -> (i32, i32)
where
    T: BufRead,
{
    let (mut part1, mut part2) = (0i32, 0i32);
    let mut group = vec![];

    for line in lines {
        let s = line.unwrap();
        let half = s.len() / 2;

        part1 += element_priority(find_element(&s[0..half], &s[half..]).unwrap());

        group.push(s);

        if group.len() == 3 {
            part2 += element_priority(common_element(&group[0], &group[1], &group[2]));
            group.clear();
        }
    }

    (part1, part2)
}

// Using fold
fn resolve<T>(lines: Lines<T>) -> (i32, i32)
where
    T: BufRead,
{
    let mut group = vec![];

    lines.fold((0i32, 0i32), |scores, line| {
        let s = line.unwrap();
        let half = s.len() / 2;

        let part1 = element_priority(find_element(&s[0..half], &s[half..]).unwrap());
        let part2;

        group.push(s);

        if group.len() == 3 {
            part2 = element_priority(common_element(&group[0], &group[1], &group[2]));
            group.clear();
        } else {
            part2 = 0;
        }
        (scores.0 + part1, scores.1 + part2)
    })
}

#[test]
fn check() {
    const TEST: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    use std::io::Cursor;

    let (part1, part2) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, 157);
    assert_eq!(part2, 70);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2022::Day::new(file!(), resolve_string) }
