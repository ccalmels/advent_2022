use std::io::{BufRead, Lines};

fn to_value(c: char) -> i32 {
    match c {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!(),
    }
}

fn to_char(v: i32) -> char {
    match v {
        -2 => '=',
        -1 => '-',
        0 => '0',
        1 => '1',
        2 => '2',
        _ => panic!(),
    }
}

fn add_snafu(a: &str, b: &str) -> String {
    let (longest, shortest) = if a.len() > b.len() { (a, b) } else { (b, a) };

    let mut retenue = 0;
    let mut ret = longest
        .chars()
        .rev()
        .zip(shortest.chars().rev().chain(std::iter::repeat('0')))
        .map(|(c1, c2)| {
            let sum = to_value(c1) + to_value(c2) + retenue;

            retenue = sum.signum() * (sum.abs() + 2) / 5;

            to_char(sum - retenue * 5)
        })
        .collect::<Vec<_>>();

    if retenue != 0 {
        ret.push(to_char(retenue));
    }

    ret.into_iter().rev().collect()
}

#[test]
fn check_add() {
    assert_eq!(add_snafu("1-", "11"), "20");
    assert_eq!(add_snafu("2-", "11"), "1=0");
    assert_eq!(add_snafu("1=0", "10"), "1-0");

    assert_eq!(add_snafu("1=11-2", "1-0"), "1=12=2");
}

fn resolve<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    (
        lines.fold(String::from("0"), |sum, line| {
            add_snafu(&sum, &line.unwrap())
        }),
        String::from(""),
    )
}

#[test]
fn check() {
    const TEST: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
    use std::io::Cursor;

    let (part1, _) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, "2=-1=0");
}

inventory::submit! { advent_2022::Day::new(file!(), resolve) }
