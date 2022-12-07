use std::cmp::{Eq, Ord, Ordering};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

#[derive(Eq, Ord)]
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

    fn parse_number(&self) -> u32 {
        self.day_filename
            .strip_suffix(".rs")
            .unwrap()
            .strip_prefix("src/day")
            .unwrap()
            .parse::<u32>()
            .unwrap()
    }

    fn resolve(&self) -> (u32, String, String) {
        let day_number = self.parse_number();
        let (part1, part2) =
            (self.resolve)(read_lines(format!("./inputs/{:0>2}.txt", day_number)).unwrap());
        (day_number, part1, part2)
    }
}

impl PartialEq for Day {
    fn eq(&self, other: &Self) -> bool {
        self.day_filename == other.day_filename
    }
}

impl PartialOrd for Day {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.day_filename.cmp(&other.day_filename))
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}

pub fn resolve_all() {
    let mut days: Vec<&'static Day> = inventory::iter::<Day>.into_iter().collect();

    days.sort();

    for day in days {
        let (day_number, part1, part2) = day.resolve();

        println!("day{:0>2}: part1: {} part2: {}", day_number, part1, part2);
    }
}

pub fn resolve_one(day_number: u32) {
    let module_name = format!("src/day{:0>2}.rs", day_number);
    let (day_number, part1, part2) = inventory::iter::<Day>
        .into_iter()
        .find(|d| d.day_filename == module_name)
        .unwrap()
        .resolve();

    println!("day{:0>2}: part1: {} part2: {}", day_number, part1, part2);
}

inventory::collect!(Day);
