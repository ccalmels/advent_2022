use std::io::{BufRead, Lines};

fn add(a: char, b: char) -> (char, char) {
    match (a, b) {
        ('0', b) => ('0', b),
        (a, '0') => ('0', a),

        ('=', '=') => ('-', '1'),
        ('=', '-') => ('-', '2'),
        ('=', '1') => ('0', '-'),
        ('=', '2') => ('0', '0'),

        ('-', '=') => ('-', '2'),
        ('-', '-') => ('0', '='),
        ('-', '1') => ('0', '0'),
        ('-', '2') => ('0', '1'),

        ('1', '=') => ('0', '-'),
        ('1', '-') => ('0', '0'),
        ('1', '1') => ('0', '2'),
        ('1', '2') => ('1', '='),

        ('2', '=') => ('0', '0'),
        ('2', '-') => ('0', '1'),
        ('2', '1') => ('1', '='),
        ('2', '2') => ('1', '-'),

        (_, _) => panic!(),
    }
}

fn add_snafu(a: &str, b: &str) -> String {
    let (longest, shortest) = if a.len() > b.len() { (a, b) } else { (b, a) };

    let mut retenue = '0';
    let mut ret = vec![];

    for (c1, c2) in longest
        .chars()
        .rev()
        .zip(shortest.chars().rev().chain(std::iter::repeat('0')))
    {
        let (r1, v) = add(c1, c2);
        let (r2, v) = add(v, retenue);

        ret.push(v);

        (_, retenue) = add(r1, r2);
    }

    if retenue != '0' {
        ret.push(retenue);
    }

    ret.into_iter().rev().collect()
}

#[test]
fn check_add() {
    assert_eq!(add('1', '1'), ('0', '2'));
    assert_eq!(add('1', '2'), ('1', '='));

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
