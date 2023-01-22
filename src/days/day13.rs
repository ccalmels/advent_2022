use std::cmp::Ordering;
use std::io::{BufRead, Lines};
use std::iter::zip;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Debug, Eq)]
struct Packet {
    list: Vec<Item>,
}

#[derive(Clone, Debug, Eq)]
enum Item {
    Packet(Packet),
    Value(usize),
}

#[derive(Debug, PartialEq, Eq)]
enum ParsePacketError {
    Incomplete,
    NotAnInt,
    Remaining,
    NoLeftBracket,
}

impl Packet {
    fn new(list: Vec<Item>) -> Self {
        Packet { list }
    }
}

impl PartialEq for Packet {
    fn eq(self: &Self, other: &Self) -> bool {
        if self.list.len() == other.list.len() {
            zip(&self.list, &other.list).all(|(a, b)| a.eq(&b))
        } else {
            false
        }
    }
}

impl Ord for Packet {
    fn cmp(self: &Self, other: &Self) -> Ordering {
        for (a, b) in zip(&self.list, &other.list) {
            let cmp = a.cmp(&b);

            if cmp != std::cmp::Ordering::Equal {
                return cmp;
            }
        }
        self.list.len().cmp(&other.list.len())
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Item {
    fn eq(self: &Self, other: &Self) -> bool {
        match self {
            Item::Value(ref v) => match other {
                Item::Value(ref vother) => v.eq(vother),
                Item::Packet(_) => false,
            },
            Item::Packet(ref p) => match other {
                Item::Value(_) => false,
                Item::Packet(ref pother) => p.eq(pother),
            },
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(self: &Self, other: &Self) -> Ordering {
        match self {
            Item::Value(ref v) => match other {
                Item::Value(ref vother) => v.cmp(vother),
                Item::Packet(ref pother) => Packet::new(vec![Item::Value(*v)]).cmp(&pother),
            },
            Item::Packet(ref p) => match other {
                Item::Value(ref vother) => p.cmp(&Packet::new(vec![Item::Value(*vother)])),
                Item::Packet(ref pother) => p.cmp(&pother),
            },
        }
    }
}

struct Cursor<'a> {
    cursor: usize,
    chars: &'a [char],
}

#[derive(Debug, PartialEq, Eq)]
struct CursorError;

impl<'a> Cursor<'a> {
    fn new(chars: &'a [char]) -> Self {
        Cursor { cursor: 0, chars }
    }

    fn has_remaining(self: &Self) -> bool {
        self.cursor < self.chars.len()
    }

    fn get(self: &Self) -> Result<char, CursorError> {
        if self.has_remaining() {
            Ok(self.chars[self.cursor])
        } else {
            Err(CursorError)
        }
    }

    fn consume(self: &mut Self) {
        self.cursor += 1;
    }

    fn next(self: &mut Self) -> Result<char, CursorError> {
        self.consume();
        self.get()
    }
}

impl From<CursorError> for ParsePacketError {
    fn from(_err: CursorError) -> Self {
        ParsePacketError::Incomplete
    }
}

fn get_packet(cursor: &mut Cursor) -> Result<Packet, ParsePacketError> {
    let mut list = vec![];
    let mut c = cursor.get()?;

    while c != ']' {
        list.push(get_item(cursor)?);
        c = cursor.get()?;

        if c == ',' {
            c = cursor.next()?;
        }
    }
    cursor.consume(); // we can be at the end
    Ok(Packet::new(list))
}

impl From<ParseIntError> for ParsePacketError {
    fn from(_err: ParseIntError) -> Self {
        ParsePacketError::NotAnInt
    }
}

fn get_item(cursor: &mut Cursor) -> Result<Item, ParsePacketError> {
    let mut value = String::from("");
    let mut c = cursor.get()?;

    if c == '[' {
        cursor.next()?;
        Ok(Item::Packet(get_packet(cursor)?))
    } else {
        while ![',', ']'].contains(&c) {
            value.push(c);
            c = cursor.next()?;
        }
        Ok(Item::Value(value.parse::<usize>()?))
    }
}

impl FromStr for Packet {
    type Err = ParsePacketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Vec<_>>();
        let mut cursor = Cursor::new(&chars);

        if cursor.get()? != '[' {
            Err(ParsePacketError::NoLeftBracket)
        } else {
            cursor.next()?;

            let p = get_packet(&mut cursor)?;

            if cursor.has_remaining() {
                Err(ParsePacketError::Remaining)
            } else {
                Ok(p)
            }
        }
    }
}

#[test]
fn check_packet() {
    let p1 = Packet::new(vec![
        Item::Value(1),
        Item::Value(1),
        Item::Value(3),
        Item::Value(1),
        Item::Value(1),
    ]);
    let p2 = Packet::new(vec![
        Item::Value(1),
        Item::Value(1),
        Item::Value(5),
        Item::Value(1),
        Item::Value(1),
    ]);

    assert_eq!(p1, "[1,1,3,1,1]".parse::<Packet>().unwrap());
    assert_eq!(p2, "[1,1,5,1,1]".parse::<Packet>().unwrap());
    assert!(p1 < p2);

    let p1 = Packet::new(vec![
        Item::Packet(Packet::new(vec![Item::Value(1)])),
        Item::Packet(Packet::new(vec![
            Item::Value(2),
            Item::Value(3),
            Item::Value(4),
        ])),
    ]);
    let p2 = Packet::new(vec![
        Item::Packet(Packet::new(vec![Item::Value(1)])),
        Item::Value(4),
    ]);

    assert_eq!(p1, "[[1],[2,3,4]]".parse::<Packet>().unwrap());
    assert_eq!(p2, "[[1],4]".parse::<Packet>().unwrap());
    assert!(p1 < p2);

    let p1 = Packet::new(vec![Item::Value(9)]);
    let p2 = Packet::new(vec![Item::Packet(Packet::new(vec![
        Item::Value(8),
        Item::Value(7),
        Item::Value(6),
    ]))]);

    assert_eq!(p1, "[9]".parse::<Packet>().unwrap());
    assert_eq!(p2, "[[8,7,6]]".parse::<Packet>().unwrap());
    assert!(p1 >= p2);

    assert!("[[4,4],4,4]".parse::<Packet>().unwrap() < "[[4,4],4,4,4]".parse::<Packet>().unwrap());

    let p1 = Packet::new(vec![Item::Packet(Packet::new(vec![Item::Packet(
        Packet::new(vec![]),
    )]))]);
    let p2 = Packet::new(vec![Item::Packet(Packet::new(vec![]))]);
    assert_eq!(p1, "[[[]]]".parse::<Packet>().unwrap());
    assert_eq!(p2, "[[]]".parse::<Packet>().unwrap());
    assert!(p1 >= p2);

    assert!(
        "[1,[2,[3,[4,[5,6,7]]]],8,9]".parse::<Packet>().unwrap()
            >= "[1,[2,[3,[4,[5,6,0]]]],8,9]".parse::<Packet>().unwrap()
    );

    // bad parsing
    assert_eq!("[1,2".parse::<Packet>(), Err(ParsePacketError::Incomplete));
    assert_eq!(
        "[1,[2,3]".parse::<Packet>(),
        Err(ParsePacketError::Incomplete)
    );
    assert_eq!(
        "[1[2,3]]".parse::<Packet>(),
        Err(ParsePacketError::NotAnInt)
    );
    assert_eq!(
        "[1x[2,3]]".parse::<Packet>(),
        Err(ParsePacketError::NotAnInt)
    );
    assert_eq!(
        "1,[2,3]]".parse::<Packet>(),
        Err(ParsePacketError::NoLeftBracket)
    );
    assert_eq!(
        "[1,[2,3]] ".parse::<Packet>(),
        Err(ParsePacketError::Remaining)
    );
    assert_eq!(
        "[1,[A,3]]".parse::<Packet>(),
        Err(ParsePacketError::NotAnInt)
    );
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut iter = lines.into_iter().peekable();
    let mut part1 = vec![];
    let mut part2 = vec![];
    let mut packets = vec![];

    while iter.peek().is_some() {
        let left = iter.next().unwrap().unwrap();
        let right = iter.next().unwrap().unwrap();

        packets.push(left.parse::<Packet>().unwrap());
        packets.push(right.parse::<Packet>().unwrap());

        iter.next();
    }

    for i in 0..packets.len() / 2 {
        let p1 = &packets[2 * i];
        let p2 = &packets[2 * i + 1];

        if p1 < p2 {
            part1.push(i + 1);
        }
    }

    let divider_packets = vec![
        "[[2]]".parse::<Packet>().unwrap(),
        "[[6]]".parse::<Packet>().unwrap(),
    ];

    for p in &divider_packets {
        packets.push(p.clone());
    }

    packets.sort();

    for (index, p) in packets.iter().enumerate() {
        if divider_packets.contains(&p) {
            part2.push(index + 1);
        }
    }

    (part1.iter().sum(), part2.iter().product())
}

#[test]
fn check() {
    const TEST: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    use std::io::Cursor;

    let (part1, part2) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, 13);
    assert_eq!(part2, 140);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2022::Day::new(file!(), resolve_string) }
