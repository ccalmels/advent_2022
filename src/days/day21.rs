use std::collections::HashMap;
use std::io::{BufRead, Lines};
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Operation {
    Add(String, String),
    Del(String, String),
    Product(String, String),
    Divide(String, String),
    Equal(String, String),
    Alias(String),
    Value(i64),
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number = s.parse::<i64>();

        if let Ok(number) = number {
            Ok(Operation::Value(number))
        } else {
            let splits = s.split(' ').collect::<Vec<_>>();
            let a = splits[0].to_string();
            let b = splits[2].to_string();

            match splits[1] {
                "+" => Ok(Operation::Add(a, b)),
                "-" => Ok(Operation::Del(a, b)),
                "*" => Ok(Operation::Product(a, b)),
                "/" => Ok(Operation::Divide(a, b)),
                _ => Err(()),
            }
        }
    }
}

fn change_root(op: Operation) -> Operation {
    match op {
        Operation::Add(a, b) => Operation::Equal(a, b),
        Operation::Del(a, b) => Operation::Equal(a, b),
        Operation::Product(a, b) => Operation::Equal(a, b),
        Operation::Divide(a, b) => Operation::Equal(a, b),
        _ => panic!(""),
    }
}

fn find_operation(hash: &HashMap<String, Operation>, name: &String) -> (String, Operation) {
    for (k, v) in hash {
        match v {
            Operation::Add(a, b) if a == name => {
                return (k.clone(), Operation::Del(k.clone(), b.clone()));
            },
            Operation::Add(a, b) if b == name => {
                return (k.clone(), Operation::Del(k.clone(), a.clone()));
            },
            Operation::Del(a, b) if a == name => {
                return (k.clone(), Operation::Add(k.clone(), b.clone()));
            },
            Operation::Del(a, b) if b == name => {
                return (k.clone(), Operation::Del(a.clone(), k.clone()));
            },
            Operation::Product(a, b) if a == name => {
                return (k.clone(), Operation::Divide(k.clone(), b.clone()));
            },
            Operation::Product(a, b) if b == name => {
                return (k.clone(), Operation::Divide(k.clone(), a.clone()));
            },
            Operation::Divide(a, b) if a == name => {
                return (k.clone(), Operation::Product(k.clone(), b.clone()));
            },
            Operation::Divide(a, b) if b == name => {
                return (k.clone(), Operation::Divide(a.clone(), k.clone()));
            },
            Operation::Equal(a, b) if a == name => {
                return (k.clone(), Operation::Alias(b.clone()));
            },
            Operation::Equal(a, b) if b == name => {
                return (k.clone(), Operation::Alias(a.clone()));
            },
            _ => continue,
        }
    }
    panic!("{name} not found");
}

fn compute(hash: &mut HashMap<String, Operation>, name: &String) -> i64 {
    let op;

    if let Some(operation) = hash.get(name) {
        op = operation.clone();
    } else {
        let key;
        (key, op) = find_operation(hash, name);

        hash.remove(&key);
    }

    let result = match op {
        Operation::Add(a, b) => compute(hash, &a) + compute(hash, &b),
        Operation::Del(a, b) => compute(hash, &a) - compute(hash, &b),
        Operation::Product(a, b) => compute(hash, &a) * compute(hash, &b),
        Operation::Divide(a, b) => compute(hash, &a) / compute(hash, &b),
        Operation::Equal(_, _)  => panic!(),
        Operation::Alias(s) => compute(hash, &s),
        Operation::Value(v) => v,
    };

    hash.insert(name.to_string(), Operation::Value(result));
    result
}

fn resolve<T>(lines: Lines<T>) -> (i64, i64)
where
    T: BufRead,
{
    let mut hash = HashMap::new();

    for line in lines {
        let line = line.unwrap();
        let splits: Vec<&str> = line.split(": ").collect();

        hash.insert(
            splits[0].to_string(),
            splits[1].parse::<Operation>().unwrap(),
        );
    }
    let mut hash_2 = hash.clone();

    let part1 = compute(&mut hash, &"root".to_string());

    hash_2.remove("humn");

    let root = hash_2.get("root").unwrap().clone();

    hash_2.insert("root".to_string(), change_root(root));

    let part2 = compute(&mut hash_2, &"humn".to_string());

    (part1, part2)
}

#[test]
fn check() {
    const TEST: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
    use std::io::Cursor;

    let (part1, part2) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, 152);
    assert_eq!(part2, 301);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2022::Day::new(file!(), resolve_string) }
