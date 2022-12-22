use std::io::{BufRead, Lines};

struct Chamber {
    grid: Vec<[bool; 7]>,
    height: usize,
    removed: usize,
}

impl Chamber {
    #[allow(dead_code)]
    fn print(self: &Self, points: Vec<(usize, usize)>) {
        for (y, row) in self.grid.iter().enumerate().rev() {
            let mut s = "".to_string();

            for (i, c) in row.iter().enumerate() {
                if *c {
                    s.push('#');
                } else {
                    if points.contains(&(i, y)) {
                        s.push('@');
                    } else {
                        s.push('.');
                    }
                }
            }
            println!("{s}");
        }
        println!("");
    }

    fn reserve(self: &mut Self, height: usize) {
        for _ in self.grid.len()..height {
            self.grid.push([false; 7]);
        }
    }

    fn check_points(self: &Self, points: Vec<(usize, usize)>) -> bool {
        points.iter().all(|&(x, y)| self.grid[y][x] == false)
    }

    fn add_points(self: &mut Self, points: Vec<(usize, usize)>) {
        points.iter().for_each(|&(x, y)| self.grid[y][x] = true);

        for (_, y) in points {
            if self.height < y {
                self.height = y;
            }

            if (0..7).all(|x| self.grid[y][x]) {
                // let l = self.grid.len();
                self.grid.drain(..y+1);
                // println!("{l} {y} new tall: {}", self.grid.len());
                self.removed += y + 1;
                self.height -= y + 1;
                return;
            }
        }
    }
}

#[derive(PartialEq)]
enum Shape {
    Horizontal,
    Cross,
    Angle,
    Vertical,
    Dot,
}

impl Shape {
    fn width(self: &Self) -> usize {
        match self {
            Shape::Horizontal => 4,
            Shape::Cross => 3,
            Shape::Angle => 3,
            Shape::Vertical => 1,
            Shape::Dot => 2,
        }
    }

    fn height(self: &Self) -> usize {
        match self {
            Shape::Horizontal => 1,
            Shape::Cross => 3,
            Shape::Angle => 3,
            Shape::Vertical => 4,
            Shape::Dot => 2,
        }
    }

    fn dots(self: &Self) -> &[(usize, usize)] {
        static HORIZONTAL: &'static [(usize, usize)] = &[(0, 0), (1, 0), (2, 0), (3, 0)];
        static CROSS: &'static [(usize, usize)] = &[(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)];
        static ANGLE: &'static [(usize, usize)] = &[(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)];
        static VERTICAL: &'static [(usize, usize)] = &[(0, 3), (0, 2), (0, 1), (0, 0)];
        static DOT: &'static [(usize, usize)] = &[(0, 1), (1, 1), (0, 0), (1, 0)];

        match self {
            Shape::Horizontal => HORIZONTAL,
            Shape::Cross => CROSS,
            Shape::Angle => ANGLE,
            Shape::Vertical => VERTICAL,
            Shape::Dot => DOT,
        }
    }

    fn next(self: &Self) -> Self {
        match self {
            Shape::Horizontal => Shape::Cross,
            Shape::Cross => Shape::Angle,
            Shape::Angle => Shape::Vertical,
            Shape::Vertical => Shape::Dot,
            Shape::Dot => Shape::Horizontal,
        }
    }
}

struct Tetris {
    pos: (usize, usize),
    shape: Shape,
}

impl Tetris {
    fn points(self: &Self) -> Vec<(usize, usize)> {
        self.shape
            .dots()
            .iter()
            .map(|(vx, vy)| (self.pos.0 + vx, self.pos.1 + vy))
            .collect()
    }

    fn move_left(self: &mut Self, chamber: &Chamber) {
        if self.pos.0 > 0 {
            let points = self
                .shape
                .dots()
                .iter()
                .map(|(x, y)| (self.pos.0 - 1 + x, self.pos.1 + y))
                .collect();

            if chamber.check_points(points) {
                self.pos.0 -= 1;
            }
        }
    }

    fn move_right(self: &mut Self, chamber: &Chamber) {
        if self.pos.0 + self.shape.width() < 7 {
            let points = self
                .shape
                .dots()
                .iter()
                .map(|(x, y)| (self.pos.0 + 1 + x, self.pos.1 + y))
                .collect();

            if chamber.check_points(points) {
                self.pos.0 += 1;
            }
        }
    }

    fn move_down(self: &mut Self, chamber: &Chamber) -> bool {
        if self.pos.1 > 0 {
            let points = self
                .shape
                .dots()
                .iter()
                .map(|(x, y)| (self.pos.0 + x, self.pos.1 - 1 + y))
                .collect();

            if chamber.check_points(points) {
                self.pos.1 -= 1;
                return true;
            }
        }
        false
    }
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut chamber = Chamber {
        grid: vec![[false; 7]; 4], height: 0, removed: 0,
    };
    let mut tetris = Tetris {
        pos: (2, 3),
        shape: Shape::Horizontal,
    };
    let jets = lines
        .into_iter()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .collect::<Vec<_>>();
    let mut jet_index = 0;
    let mut rocks = 0usize;

    // chamber.print(tetris.points());

    loop {
        let dir = jets[jet_index];

        jet_index = (jet_index + 1) % jets.len();

        if dir == '<' {
            tetris.move_left(&chamber);
        } else {
            tetris.move_right(&chamber);
        }

        // chamber.print(tetris.points());

        if !tetris.move_down(&chamber) {
            chamber.add_points(tetris.points());

            rocks += 1;
            if rocks == 2022 {
                break;
            }

            tetris.shape = tetris.shape.next();

            if jet_index == 0 && tetris.shape == Shape::Horizontal {
                println!("loop");
            }

            chamber.reserve(chamber.height + 4 + tetris.shape.height());

            tetris.pos = (2, chamber.height + 4);
        }
        //chamber.print(tetris.points());
    }

    (chamber.height + chamber.removed + 1, 0)
}

#[test]
fn check() {
    const TEST: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    use std::io::Cursor;

    let (part1, _part2) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, 3068);
//    assert_eq!(part2, 1514285714288);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2022::Day::new(file!(), resolve_string) }
