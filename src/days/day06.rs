use std::io::{BufRead, Lines};

fn get_index(buf_slice: &[u8]) -> Option<usize> {
    for (i, c) in buf_slice.iter().enumerate().rev() {
        for j in 0..i {
            if *c == buf_slice[j] {
                return Some(j + 1);
            }
        }
    }
    None
}

fn find_first_index(buffer: &[u8], distincts: usize) -> usize {
    let mut index = 0;

    while index < buffer.len() - distincts {
        let idx = get_index(&buffer[index..index + distincts]);

        if let Some(idx) = idx {
            index += idx;
        } else {
            return index + distincts;
        }
    }
    panic!("");
}

fn resolve<T>(lines: Lines<T>) -> (Vec<usize>, Vec<usize>)
where
    T: BufRead,
{
    let mut part1 = vec![];
    let mut part2 = vec![];

    for line in lines {
        let line = line.unwrap();
        let buf = line.as_bytes();

        part1.push(find_first_index(buf, 4));
        part2.push(find_first_index(buf, 14));
    }

    (part1, part2)
}

#[test]
fn check() {
    const TEST: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb
bvwbjplbgvbhsrlpgdmjqwftvncz
nppdvjthqldpwncqszvftbrmjlhg
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    use std::io::Cursor;

    let (part1, part2) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, [7, 5, 6, 10, 11]);
    assert_eq!(part2, [19, 23, 23, 29, 26]);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0[0].to_string(), solution.1[0].to_string())
}

inventory::submit! { advent_2022::Day::new(file!(), resolve_string) }
