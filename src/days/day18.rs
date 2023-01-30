use regex::Regex;
use std::collections::HashSet;
use std::io::{BufRead, Lines};

fn get_neighbor(p: (i32, i32, i32)) -> [(i32, i32, i32); 6] {
    [
        (p.0 + 1, p.1, p.2),
        (p.0 - 1, p.1, p.2),
        (p.0, p.1 + 1, p.2),
        (p.0, p.1 - 1, p.2),
        (p.0, p.1, p.2 + 1),
        (p.0, p.1, p.2 - 1),
    ]
}

fn is_adjacent(p: (i32, i32, i32), hash: &HashSet<(i32, i32, i32)>) -> bool {
    get_neighbor(p).iter().any(|&p| hash.contains(&p))
}

fn count_faces(mut points: HashSet<(i32, i32, i32)>) -> usize {
    let mut adjacents = 6 * points.len();

    while !points.is_empty() {
        let p = *points.iter().next().unwrap();

        points.remove(&p);

        adjacents -= 2 * get_neighbor(p)
            .iter()
            .filter(|&p| points.contains(p))
            .count();
    }

    adjacents
}

fn capture_point(re: &regex::Regex, line: &str) -> (i32, i32, i32) {
    let point_capture = re.captures(line).unwrap();
    let x = point_capture
        .get(1)
        .unwrap()
        .as_str()
        .parse::<i32>()
        .unwrap();
    let y = point_capture
        .get(2)
        .unwrap()
        .as_str()
        .parse::<i32>()
        .unwrap();
    let z = point_capture
        .get(3)
        .unwrap()
        .as_str()
        .parse::<i32>()
        .unwrap();
    (x, y, z)
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let point_regex = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();
    let mut points = HashSet::new();
    let mut max = (0, 0, 0);

    for line in lines {
        let p = capture_point(&point_regex, &line.unwrap());

        max.0 = i32::max(max.0, p.0);
        max.1 = i32::max(max.1, p.1);
        max.2 = i32::max(max.2, p.2);

        points.insert(p);
    }

    let mut spaces = HashSet::new();
    let mut external = HashSet::new();

    for x in 0..max.0 + 1 {
        for y in 0..max.1 + 1 {
            for z in 0..max.2 + 1 {
                let p = (x, y, z);

                if !points.contains(&p) {
                    if p.0 == 0
                        || p.1 == 0
                        || p.2 == 0
                        || p.0 == max.0
                        || p.1 == max.1
                        || p.2 == max.2
                        || is_adjacent(p, &external)
                    {
                        external.insert(p);
                    } else {
                        spaces.insert(p);
                    }
                }
            }
        }
    }

    // println!("external/spaces {}/{}", external.len(), spaces.len());

    loop {
        let n = external.len();

        spaces.retain(|&x| {
            if is_adjacent(x, &external) {
                external.insert(x);
                false
            } else {
                true
            }
        });

        if external.len() == n {
            break;
        }
    }

    // println!("external/spaces {}/{}", external.len(), spaces.len());

    let faces = count_faces(points);

    (faces, faces - count_faces(spaces))
}

#[test]
fn check() {
    const TEST: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
    use std::io::Cursor;

    let (part1, part2) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, 64);
    assert_eq!(part2, 58);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2022::Day::new(file!(), resolve_string) }
