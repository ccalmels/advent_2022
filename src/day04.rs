use std::io::{BufRead, Lines};

fn range_contains(a: (u32, u32), b: (u32, u32)) -> bool {
    !((a.0 > b.0 || a.1 < b.1) && (b.0 > a.0 || b.1 < a.1))
}

#[test]
fn check_contains() {
    assert_eq!(range_contains((2, 4), (6, 8)), false);
    assert_eq!(range_contains((2, 3), (4, 5)), false);
    assert_eq!(range_contains((5, 7), (7, 9)), false);
    assert_eq!(range_contains((2, 8), (3, 7)), true);
    assert_eq!(range_contains((6, 6), (4, 6)), true);
    assert_eq!(range_contains((2, 6), (4, 8)), false);
}

fn range_overlap(a: (u32, u32), b: (u32, u32)) -> bool {
    !((a.1 < b.0) || (b.1 < a.0))
}

#[test]
fn check_overlap() {
    assert_eq!(range_overlap((2, 4), (6, 8)), false);
    assert_eq!(range_overlap((2, 3), (4, 5)), false);
    assert_eq!(range_overlap((5, 7), (7, 9)), true);
    assert_eq!(range_overlap((2, 8), (3, 7)), true);
    assert_eq!(range_overlap((6, 6), (4, 6)), true);
    assert_eq!(range_overlap((2, 6), (4, 8)), true);
}

fn resolve<T>(lines: Lines<T>) -> (u32, u32)
where
    T: BufRead,
{
    lines.fold((0u32, 0u32), |scores, line| {
        let values: Vec<u32> = line
            .unwrap()
            .split(|c: char| !c.is_ascii_digit())
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let (a, b) = ((values[0], values[1]), (values[2], values[3]));

        (
            scores.0 + range_contains(a, b) as u32,
            scores.1 + range_overlap(a, b) as u32,
        )
    })
}

#[test]
fn check() {
    const TEST: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    use std::io::Cursor;

    let (part1, part2) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, 2);
    assert_eq!(part2, 4);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2022::Day::new(file!(), resolve_string) }
