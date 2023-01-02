use regex::Regex;
use std::io::{BufRead, Lines};
use std::collections::HashSet;

fn are_adjacent(a: (i32, i32, i32), b: (i32, i32, i32)) -> bool {
    let x = (b.0 - a.0).abs();
    let y = (b.1 - a.1).abs();
    let z = (b.2 - a.2).abs();

    x + y + z == 1
}

fn is_adjacent(p: (i32, i32, i32), list: &Vec<(i32, i32, i32)>) -> bool {
    list.iter().any(|&x| are_adjacent(p, x))
}

fn count_faces(points: &Vec<(i32, i32, i32)>) -> usize {
    let mut adjacents = 0;

    for i in 0..points.len() {
        for j in i..points.len() {
            if are_adjacent(points[i], points[j]) {
                adjacents += 1;
            }
        }
    }

    points.len() * 6 - adjacents * 2
}

fn capture_point(re: &regex::Regex, line: &String) -> (i32, i32, i32) {
    let point_capture = re.captures(&line).unwrap();
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
    let mut points = vec![];
    let mut max = (0, 0, 0);

    for line in lines {
        let p = capture_point(&point_regex, &line.unwrap());

        max.0 = i32::max(max.0, p.0);
        max.1 = i32::max(max.1, p.1);
        max.2 = i32::max(max.2, p.2);

        points.push(p);
    }

    let mut spaces = vec![];
    let mut external = vec![];

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
                        external.push(p);
                    } else {
                        spaces.push(p);
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
                external.push(x);
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

    let faces = count_faces(&points);

    (faces, faces - count_faces(&spaces))
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
