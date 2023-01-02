use std::io::{BufRead, Lines};

#[derive(Clone, Copy, Debug)]
struct Elem {
    prev: usize,
    next: usize,
}

impl Elem {
    fn new(prev: usize, next: usize) -> Self {
        Elem { prev, next }
    }
}

fn move_step(indexes: &Vec<Elem>, index: usize, forward: bool, count: usize) -> usize {
    let mut v = index;

    if forward {
        for _ in 0..count {
            v = indexes[v].next;
        }
    } else {
        for _ in 0..count {
            v = indexes[v].prev;
        }
    }

    v
}

fn mix(indexes: &mut Vec<Elem>, values: &Vec<i64>, key: i64) {
    for (index, value) in values.iter().enumerate() {
        let forward;
        let mut steps = (value * key).rem_euclid(indexes.len() as i64 - 1) as usize;

        if steps == 0 {
            continue;
        }
        if steps > (indexes.len() - 1) / 2 {
            forward = false;
            steps = indexes.len() - 1 - steps;
        } else {
            forward = true;
        }
        let elem = indexes[index];

        // remove elem
        indexes[elem.prev].next = elem.next;
        indexes[elem.next].prev = elem.prev;

        let after = move_step(&indexes, elem.next, forward, steps);
        let before = indexes[after].prev;

        // println!("{} in {}, {}", value, values[after], values[before]);

        indexes[after].prev = index;
        indexes[before].next = index;
        indexes[index].prev = before;
        indexes[index].next = after;
    }
}

fn create_indexes(length: usize) -> Vec<Elem> {
    let mut indexes = vec![];

    for i in 0..length {
        let (prev, next);

        if i == 0 {
            prev = length - 1;
        } else {
            prev = i - 1;
        }
        if i == length - 1 {
            next = 0;
        } else {
            next = i + 1;
        }

        indexes.push(Elem::new(prev, next));
    }
    indexes
}

fn resolve<T>(lines: Lines<T>) -> (i64, i64)
where
    T: BufRead,
{
    let mut values = vec![];
    let mut v0_index = 0;
    let key = 811589153;

    for line in lines {
        let n = line.unwrap().parse::<i64>().unwrap();

        if n == 0 {
            v0_index = values.len();
        }

        values.push(n);
    }

    let mut indexes = create_indexes(values.len());
    mix(&mut indexes, &values, 1);

    let v1000_index = move_step(&indexes, v0_index, true, 1000);
    let v2000_index = move_step(&indexes, v1000_index, true, 1000);
    let v3000_index = move_step(&indexes, v2000_index, true, 1000);
    let part1 = values[v1000_index] + values[v2000_index] + values[v3000_index];

    indexes = create_indexes(values.len());
    for _ in 0..10 {
        mix(&mut indexes, &values, key);
    }

    let v1000_index = move_step(&indexes, v0_index, true, 1000);
    let v2000_index = move_step(&indexes, v1000_index, true, 1000);
    let v3000_index = move_step(&indexes, v2000_index, true, 1000);
    let part2 = (values[v1000_index] + values[v2000_index] + values[v3000_index]) * key;

    (part1, part2)
}

#[test]
fn check() {
    const TEST: &str = "1
2
-3
3
-2
0
4";
    use std::io::Cursor;

    let (part1, part2) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, 3);
    assert_eq!(part2, 1623178306);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2022::Day::new(file!(), resolve_string) }
