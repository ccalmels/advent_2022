use std::io::{BufRead, Lines};

fn resolve<T>(lines: Lines<T>) -> (u32, u32)
where
    T: BufRead,
{
    let mut elves = vec![];
    let mut elve_calories = 0;

    for line in lines {
        let line = line.unwrap();
        if let Ok(calories) = line.parse::<u32>() {
            elve_calories += calories;
        } else {
            elves.push(elve_calories);
            elve_calories = 0;
        }
    }
    elves.push(elve_calories);

    elves.sort_by(|a, b| b.cmp(a));

    (elves[0], elves[0..3].iter().sum())
}

#[test]
fn check() {
    const TEST: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    use std::io::Cursor;

    let (part1, part2) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, 24000);
    assert_eq!(part2, 45000);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2022::Day::new(file!(), resolve_string) }
