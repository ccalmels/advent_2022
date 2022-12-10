use std::collections::HashSet;
use std::io::{BufRead, Lines};

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn vector(a: &Point, b: &Point) -> (i32, i32) {
    (b.x - a.x, b.y - a.y)
}

impl Point {
    fn new() -> Self {
        Point { x: 0, y: 0 }
    }

    fn add(self: &mut Self, v: (i32, i32)) {
        self.x += v.0;
        self.y += v.1;
    }

    fn follow(self: &mut Self, other: &Point) {
        let (vx, vy) = vector(self, other);

        if !(vx.abs() < 2 && vy.abs() < 2) {
            self.add((vx.signum(), vy.signum()));
        }
    }
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let size = 9;
    let mut head = Point::new();
    let mut tail = Point::new();
    let mut knots = vec![Point::new(); size];
    let mut part1 = HashSet::new();
    let mut part2 = HashSet::new();

    for line in lines {
        let line = line.unwrap();
        let mut split = line.split(' ');
        let direction = split.next().unwrap();
        let value = split.next().unwrap().parse::<usize>().unwrap();

        let vector = match direction {
            "L" => (-1, 0),
            "R" => (1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => panic!(),
        };

        for _ in 0..value {
            head.add(vector);

            // part 1
            tail.follow(&head);

            part1.insert((tail.x, tail.y));

            // part 2
            knots[0].follow(&head);

            for i in 1..size {
                let previous = knots[i - 1].clone();

                knots[i].follow(&previous);
            }

            part2.insert((knots[size - 1].x, knots[size - 1].y));
        }
    }

    (part1.len(), part2.len())
}

#[test]
fn check() {
    const TEST: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    use std::io::Cursor;

    let (part1, part2) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, 13);
    assert_eq!(part2, 1);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2022::Day::new(file!(), resolve_string) }
