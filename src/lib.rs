use std::cmp::{Eq, Ord, Ordering};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use std::time::Instant;

#[derive(Eq)]
pub struct Day {
    day_filename: &'static str,
    resolve: fn(Lines<BufReader<File>>) -> (String, String),
}

impl Day {
    pub const fn new(
        day_filename: &'static str,
        resolve: fn(Lines<BufReader<File>>) -> (String, String),
    ) -> Self {
        Day {
            day_filename,
            resolve,
        }
    }

    fn print(&self) {
        let start = Instant::now();
        let (day_number, part1, part2) = self.resolve();
        let duration = start.elapsed();

        println!("day{day_number:0>2}: part1: {part1:20} part2: {part2:20} in {duration:?}");
    }

    fn parse_number(&self) -> u32 {
        self.day_filename
            .replace(|c: char| !c.is_ascii_digit(), "")
            .parse::<u32>()
            .unwrap()
    }

    fn resolve(&self) -> (u32, String, String) {
        let day_number = self.parse_number();
        let (part1, part2) =
            (self.resolve)(read_lines(format!("./inputs/{day_number:0>2}.txt")).unwrap());
        (day_number, part1, part2)
    }
}

impl PartialEq for Day {
    fn eq(&self, other: &Self) -> bool {
        self.day_filename == other.day_filename
    }
}

impl Ord for Day {
    fn cmp(&self, other: &Self) -> Ordering {
        self.day_filename.cmp(other.day_filename)
    }
}
impl PartialOrd for Day {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.day_filename.partial_cmp(other.day_filename)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}

fn resolve_all() {
    let mut days: Vec<&'static Day> = inventory::iter::<Day>.into_iter().collect();

    days.sort_unstable();

    days.iter().for_each(|d| d.print());
}

fn resolve_one(day_number: u32) {
    inventory::iter::<Day>
        .into_iter()
        .find(|d| d.parse_number() == day_number)
        .unwrap()
        .print();
}

pub fn resolve(days: &[u32]) {
    let start = Instant::now();

    if days.is_empty() {
        resolve_all();
    } else {
        days.iter().for_each(|&d| resolve_one(d));
    }

    let duration = start.elapsed();

    println!("All done in {duration:?}");
}

inventory::collect!(Day);
