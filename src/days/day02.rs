use std::io::{BufRead, Lines};

#[derive(PartialEq)]
enum Shape {
    Rock,
    Paper,
    Cissor,
}

impl Shape {
    fn value(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Cissor => 3,
        }
    }

    fn looser(&self) -> Self {
        match self {
            Shape::Rock => Shape::Cissor,
            Shape::Paper => Shape::Rock,
            Shape::Cissor => Shape::Paper,
        }
    }

    fn winner(&self) -> Self {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Cissor,
            Shape::Cissor => Shape::Rock,
        }
    }
}

#[test]
fn check_shape() {
    let rock = Shape::Rock;
    let paper = Shape::Paper;
    let cissor = Shape::Cissor;

    assert_eq!(rock.value(), 1);
    assert_eq!(paper.value(), 2);
    assert_eq!(cissor.value(), 3);

    assert!(rock == Shape::Rock);
    assert!(rock.looser() == Shape::Cissor);
    assert!(rock.winner() == Shape::Paper);

    assert!(paper == Shape::Paper);
    assert!(paper.looser() == Shape::Rock);
    assert!(paper.winner() == Shape::Cissor);

    assert!(cissor == Shape::Cissor);
    assert!(cissor.looser() == Shape::Paper);
    assert!(cissor.winner() == Shape::Rock);

    assert!(rock != Shape::Paper);
}

fn round_score_part1(elve: &Shape, strategy: &str) -> u32 {
    let me = match strategy {
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Cissor,
        _ => panic!(),
    };

    if me == *elve {
        3 + me.value()
    } else if me.looser() == *elve {
        6 + me.value()
    } else {
        me.value()
    }
}

fn round_score_part2(elve: &Shape, strategy: &str) -> u32 {
    match strategy {
        "X" => elve.looser().value(),
        "Y" => 3 + elve.value(),
        "Z" => 6 + elve.winner().value(),
        _ => panic!(),
    }
}

// First try
fn _resolve<T>(lines: Lines<T>) -> (u32, u32)
where
    T: BufRead,
{
    let mut scores = (0u32, 0u32);

    for line in lines {
        if let Ok(s) = line {
            let words = s.split(" ").collect::<Vec<_>>();
            let elve = match words[0] {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Cissor,
                _ => panic!(),
            };

            scores.0 += round_score_part1(&elve, words[1]);
            scores.1 += round_score_part2(&elve, words[1]);
        }
    }
    scores
}

// Using fold
fn resolve<T>(lines: Lines<T>) -> (u32, u32)
where
    T: BufRead,
{
    lines.fold((0u32, 0u32), |scores, line| {
        let words = line.as_ref().unwrap().split(" ").collect::<Vec<_>>();
        let elve = match words[0] {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Cissor,
            _ => panic!(),
        };

        (
            scores.0 + round_score_part1(&elve, words[1]),
            scores.1 + round_score_part2(&elve, words[1]),
        )
    })
}

#[test]
fn check() {
    const TEST: &str = "A Y
B X
C Z";
    use std::io::Cursor;

    let (part1, part2) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, 15);
    assert_eq!(part2, 12);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2022::Day::new(file!(), resolve_string) }
