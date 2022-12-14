use regex::Regex;
use std::io::{BufRead, Lines};

#[derive(Debug, Clone)]
enum Operation {
    Multiply(u64),
    Add(u64),
    Square(),
}

impl Operation {
    fn compute(self: &Self, value: u64) -> u64 {
        match self {
            Operation::Multiply(v) => value * v,
            Operation::Add(v) => value + v,
            Operation::Square() => value * value,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisible: (u64, usize, usize),
    inspects: u64,
}

impl Monkey {
    fn new(items: Vec<u64>, operation: Operation, divisible: (u64, usize, usize)) -> Self {
        Monkey {
            items,
            operation,
            divisible,
            inspects: 0,
        }
    }

    fn round<F>(self: &mut Self, reduce: F) -> Vec<(u64, usize)>
    where
        F: Fn(u64) -> u64,
    {
        let mut throws = vec![];

        for item in &self.items {
            let item = reduce(self.operation.compute(*item));

            if item % self.divisible.0 == 0 {
                throws.push((item, self.divisible.1));
            } else {
                throws.push((item, self.divisible.2));
            }
            self.inspects += 1;
        }

        self.items.clear();

        throws
    }
}

fn get_next_number<T, U>(iter: &mut std::io::Lines<T>) -> U
where
    T: BufRead,
    U: std::str::FromStr,
    <U as std::str::FromStr>::Err: std::fmt::Debug,
{
    let number_regex = Regex::new(r"\d+").unwrap();
    let n = iter.next().unwrap().unwrap();
    number_regex
        .find(&n)
        .unwrap()
        .as_str()
        .parse::<U>()
        .unwrap()
}

fn read_monkeys<T>(lines: Lines<T>) -> Vec<Monkey>
where
    T: BufRead,
{
    let number_regex = Regex::new(r"\d+").unwrap();
    let operation_regex = Regex::new(r"new = old (\*|\+) (\d+|old)").unwrap();
    let mut iter = lines.into_iter();
    let mut monkeys = vec![];

    while !iter.next().is_none() {
        let items = iter.next().unwrap().unwrap();
        let items = number_regex
            .find_iter(&items)
            .map(|x| x.as_str().parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let operation = iter.next().unwrap().unwrap();
        let operation_capture = operation_regex.captures(&operation).unwrap();
        let op = operation_capture.get(1).unwrap().as_str();
        let v = operation_capture.get(2).unwrap().as_str();
        let operation = match &v {
            &"old" => Operation::Square(),
            _ => {
                let v = v.parse::<u64>().unwrap();
                match op {
                    "*" => Operation::Multiply(v),
                    "+" => Operation::Add(v),
                    _ => panic!(),
                }
            }
        };
        let divisible = get_next_number::<T, u64>(&mut iter);
        let if_true = get_next_number::<T, usize>(&mut iter);
        let if_false = get_next_number::<T, usize>(&mut iter);

        iter.next();

        monkeys.push(Monkey::new(
            items,
            operation,
            (divisible, if_true, if_false),
        ));
    }
    monkeys
}

fn resolve<T>(lines: Lines<T>) -> (u64, u64)
where
    T: BufRead,
{
    let mut monkeys_1 = read_monkeys(lines);
    let mut monkeys_2 = monkeys_1.clone();
    let len = monkeys_1.len();

    for i in 0..len * 20 {
        let throws = monkeys_1[i % len].round(|x| x / 3);

        for (item, index) in throws {
            monkeys_1[index].items.push(item);
        }
    }

    let mut inspects_1 = monkeys_1.iter().map(|x| x.inspects).collect::<Vec<_>>();
    inspects_1.sort();

    let supermodulo: u64 = monkeys_2.iter().map(|m| m.divisible.0).product();

    for i in 0..len * 10000 {
        let throws = monkeys_2[i % len].round(|x| x % supermodulo); //_part2(supermodulo);

        for (item, index) in throws {
            monkeys_2[index].items.push(item);
        }
    }

    // println!("{monkeys2:?}");

    let mut inspects_2 = monkeys_2.iter().map(|x| x.inspects).collect::<Vec<_>>();
    inspects_2.sort();

    // println!("{inspects2:?}");

    (
        inspects_1[len - 1] * inspects_1[len - 2],
        inspects_2[len - 1] * inspects_2[len - 2],
    )
}

#[test]
fn check() {
    const TEST: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
    use std::io::Cursor;

    let (part1, part2) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, 10605);
    assert_eq!(part2, 2713310158);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2022::Day::new(file!(), resolve_string) }
