use std::io::{BufRead, Lines};

fn get_max(heights: &[Option<usize>; 10]) -> u8 {
    for (i, height) in heights.iter().enumerate().rev() {
        if height.is_some() {
            return i as u8;
        }
    }
    0u8
}

fn get_visible_index(heights: &[Option<usize>; 10], value: u8) -> usize {
    let mut index = 0;

    for height in heights.iter().skip(value as usize).flatten() {
        if height > &index {
            index = *height
        }
    }
    index
}

#[derive(Debug)]
struct Visibility {
    max: u8,
    count: usize,
}

impl Visibility {
    fn new() -> Self {
        Visibility { max: 0, count: 0 }
    }

    fn compute(&mut self, heights: &[Option<usize>; 10], value: u8, index: usize) {
        self.max = get_max(heights);
        self.count = index - get_visible_index(heights, value);
    }
}

#[derive(Debug)]
struct Tree {
    value: u8,
    left: Visibility,
    right: Visibility,
    up: Visibility,
    down: Visibility,
}

impl Tree {
    fn new(value: u8) -> Self {
        Tree {
            value,
            left: Visibility::new(),
            right: Visibility::new(),
            up: Visibility::new(),
            down: Visibility::new(),
        }
    }

    fn is_visible(&self) -> bool {
        self.value > self.left.max
            || self.value > self.right.max
            || self.value > self.up.max
            || self.value > self.down.max
    }

    fn scenic_score(&self) -> usize {
        self.left.count * self.right.count * self.up.count * self.down.count
    }
}

fn compute_max_and_visible(grid: &mut Vec<Vec<Tree>>) {
    for row in grid.iter_mut() {
        let mut last_indices_left = [None; 10];
        let mut last_indices_right = [None; 10];
        let len = row.len();

        for i in 0..len {
            let tree = &mut row[i];

            tree.left.compute(&last_indices_left, tree.value, i);

            last_indices_left[tree.value as usize] = Some(i);

            let tree = &mut row[len - 1 - i];

            tree.right.compute(&last_indices_right, tree.value, i);

            last_indices_right[tree.value as usize] = Some(i);
        }
    }

    for i in 0..grid[0].len() {
        let mut last_indices_up = [None; 10];
        let mut last_indices_down = [None; 10];
        let len = grid.len();

        for j in 0..len {
            let tree = &mut grid[j][i];

            tree.up.compute(&last_indices_up, tree.value, j);

            last_indices_up[tree.value as usize] = Some(j);

            let tree = &mut grid[len - 1 - j][i];

            tree.down.compute(&last_indices_down, tree.value, j);

            last_indices_down[tree.value as usize] = Some(j);
        }
    }
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut grid = vec![];

    for line in lines {
        let line = line.unwrap();
        let array: Vec<Tree> = line
            .as_bytes()
            .iter()
            .map(|x| Tree::new(x - b'0'))
            .collect();

        grid.push(array);
    }

    compute_max_and_visible(&mut grid);

    // println!("{grid:?}");
    // println!("{:?}", grid[1][2]);
    // println!("{:?}", grid[3][2]);

    let mut part1 = 4 * (grid.len() - 1);
    let mut part2 = 0;

    for j in 0..grid.len() {
        for i in 0..grid[j].len() {
            let score = grid[j][i].scenic_score();

            part2 = std::cmp::max(part2, score);

            if i > 0 && j > 0 && i < grid[j].len() - 1 && j < grid.len() - 1 {
                part1 += grid[j][i].is_visible() as usize;
            }
        }
    }

    (part1, part2)
}

#[test]
fn check() {
    const TEST: &str = "30373
25512
65332
33549
35390";
    use std::io::Cursor;

    let (part1, part2) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, 21);
    assert_eq!(part2, 8);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2022::Day::new(file!(), resolve_string) }
