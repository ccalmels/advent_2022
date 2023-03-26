use regex::Regex;
use std::collections::HashMap;
use std::io::{BufRead, Lines};

#[derive(Debug)]
struct Valve {
    name: String,
    rate: u32,
    neighbors: Vec<String>,
}

impl Valve {
    fn new(name: &str, rate: u32, neighbors: Vec<String>) -> Self {
        let name = name.to_string();
        Valve {
            name,
            rate,
            neighbors,
        }
    }
}

type Mask = u64;

fn is_set(m: Mask, bit: usize) -> bool {
    m & (1 << bit) != 0
}

fn add(m: Mask, bit: usize) -> Mask {
    m | (1 << bit)
}

fn is_disjoint(m1: Mask, m2: Mask) -> bool {
    (m1 & m2) == 0
}

// Floyd-Warshall
fn compute_distances(valves: &Vec<Valve>) -> Vec<Vec<u32>> {
    let names: HashMap<String, usize> = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.name.to_owned(), i))
        .collect();
    let length = valves.len();
    let mut distances = vec![vec![length as u32 + 1; length]; length];

    for (i, v) in valves.iter().enumerate() {
        v.neighbors
            .iter()
            .for_each(|n| distances[i][*(names.get(n).unwrap())] = 1);
        distances[i][i] = 0;
    }

    for k in 0..length {
        for i in 0..length {
            for j in 0..length {
                if distances[i][j] > distances[i][k] + distances[k][j] {
                    distances[i][j] = distances[i][k] + distances[k][j];
                }
            }
        }
    }

    distances
}

struct Volcano {
    valves: Vec<Valve>,
    useful: Vec<usize>,
    distances: Vec<Vec<u32>>,
}

impl Volcano {
    fn new(valves: Vec<Valve>, useful: Vec<usize>, distances: Vec<Vec<u32>>) -> Self {
        Volcano {
            valves,
            useful,
            distances,
        }
    }

    fn dfs(
        &self,
        current: usize,
        opened: Mask,
        flow: u32,
        rate: u32,
        remaining_time: u32,
    ) -> Vec<(Mask, u32)> {
        let rest = flow + rate * remaining_time;
        let mut ret: Vec<(Mask, u32)> = vec![(opened, rest)];

        for &v in &self.useful {
            if is_set(opened, v) {
                continue;
            }

            let open_duration = self.distances[current][v] + 1;
            if open_duration >= remaining_time {
                continue;
            }

            ret.extend(
                self.dfs(
                    v,
                    add(opened, v),
                    flow + open_duration * rate,
                    rate + self.valves[v].rate,
                    remaining_time - open_duration,
                )
                .iter(),
            );
        }

        ret
    }
}

fn resolve<T>(lines: Lines<T>) -> (u32, u32)
where
    T: BufRead,
{
    let valve_regex = Regex::new(
        r"Valve (\w+) has flow rate=(\d+); (?:tunnels lead to valves|tunnel leads to valve) (.*)",
    )
    .unwrap();
    let valves = lines
        .map(|line| {
            let line = line.unwrap();
            let valve_capture = valve_regex.captures(&line).unwrap();
            let name = valve_capture.get(1).unwrap().as_str();
            let rate = valve_capture
                .get(2)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();
            let neighbors = valve_capture
                .get(3)
                .unwrap()
                .as_str()
                .split(", ")
                .map(String::from)
                .collect();

            Valve::new(name, rate, neighbors)
        })
        .collect::<Vec<_>>();

    if valves.len() > 64 {
        panic!("too much valves for Mask");
    }

    let distances = compute_distances(&valves);
    let useful_valves_indexes = valves
        .iter()
        .enumerate()
        .filter_map(|(i, v)| if v.rate == 0 { None } else { Some(i) })
        .collect::<Vec<_>>();
    let index_aa = valves
        .iter()
        .enumerate()
        .find_map(|(i, v)| if v.name == "AA" { Some(i) } else { None })
        .unwrap();
    let volcano = Volcano::new(valves, useful_valves_indexes, distances);

    let (_, part1) = *volcano
        .dfs(index_aa, 0, 0, 0, 30)
        .iter()
        .max_by_key(|(_, f)|f)
        .unwrap();
    let flows = volcano.dfs(index_aa, 0, 0, 0, 26);

    let mut part2 = 0;
    let (_, max_for_one) = flows.iter().max_by_key(|(_, f)| f).unwrap();

    for i in 0..flows.len() {
        let (p_i, flow_i) = &flows[i];

        if max_for_one + flow_i < part2 {
            continue;
        }

        for (p_j, flow_j) in flows.iter().skip(i+1) {
            let total_flow = flow_i + flow_j;

            if total_flow > part2 && is_disjoint(*p_i, *p_j) {
                part2 = total_flow;
            }
        }
    }

    (part1, part2)
}

#[test]
fn check() {
    const TEST: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
    use std::io::Cursor;

    let (part1, part2) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, 1651);
    assert_eq!(part2, 1707);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2022::Day::new(file!(), resolve_string) }
