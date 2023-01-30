use std::io::{BufRead, Lines};

struct Cpu {
    x: i32,
    cycle: u32,
    signal_strength: i32,
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            x: 1,
            cycle: 0,
            signal_strength: 0,
        }
    }
    fn tick(&mut self) {
        self.cycle += 1;

        if (self.cycle + 20) % 40 == 0 {
            self.signal_strength += self.x * self.cycle as i32;
        }
    }

    fn addx(&mut self, value: i32) {
        self.x += value;
    }
}

struct Crt {
    screen: String,
    pos: i32,
}

impl Crt {
    fn new() -> Self {
        Crt {
            screen: "\n".to_string(),
            pos: 0,
        }
    }

    fn tick(&mut self, x: i32) {
        if self.pos > x + 1 || self.pos < x - 1 {
            self.screen.push('.');
        } else {
            self.screen.push('#');
        }

        self.pos += 1;

        if self.pos % 40 == 0 {
            self.pos = 0;
            self.screen.push('\n');
        }
    }
}

fn resolve<T>(lines: Lines<T>) -> (i32, String)
where
    T: BufRead,
{
    let mut cpu = Cpu::new();
    let mut crt = Crt::new();

    for line in lines {
        let line = line.unwrap();
        let value = line.split(' ').nth(1);

        cpu.tick();
        crt.tick(cpu.x);

        if let Some(value) = value {
            // addx
            cpu.tick();
            crt.tick(cpu.x);

            cpu.addx(value.parse::<i32>().unwrap());
        }
    }

    (cpu.signal_strength, crt.screen)
}

#[test]
fn check() {
    const TEST: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    const RESULT: &str = "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
    use std::io::Cursor;

    let (part1, part2) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, 13140);
    assert_eq!(part2, RESULT);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1)
}

inventory::submit! { advent_2022::Day::new(file!(), resolve_string) }
