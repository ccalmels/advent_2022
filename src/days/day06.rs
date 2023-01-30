use std::io::{BufRead, Lines};

fn get_index(buf_slice: &[u8]) -> Option<usize> {
    for (i, c1) in buf_slice.iter().enumerate().rev() {
        for (j, c2) in buf_slice.iter().take(i).enumerate() {
            if c1 == c2 {
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

fn resolve<T>(lines: Lines<T>) -> Vec<(usize, usize)>
where
    T: BufRead,
{
    lines
        .map(|line| {
            let line = line.unwrap();
            let buf = line.as_bytes();

            (find_first_index(buf, 4), find_first_index(buf, 14))
        })
        .collect()
}

#[test]
fn check() {
    const TEST: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb
bvwbjplbgvbhsrlpgdmjqwftvncz
nppdvjthqldpwncqszvftbrmjlhg
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    use std::io::Cursor;

    let parts = resolve(Cursor::new(TEST).lines());

    assert_eq!(parts, [(7, 19), (5, 23), (6, 23), (10, 29), (11, 26)])
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution[0].0.to_string(), solution[0].1.to_string())
}

inventory::submit! { advent_2022::Day::new(file!(), resolve_string) }
