extern crate utils;

use utils::basic_parser;
use utils::parse_input_to_vec;

enum Direction {
    Up,
    Down,
    Forward,
}

struct Command {
    direction: Direction,
    length: i64,
}

struct Ship {
    x: i64,
    y: i64,
    aim: i64,
}

impl Ship {
    fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.aim = 0;
    }

    fn steer_problem1(&mut self, cmd: &Command) {
        match cmd.direction {
            Direction::Up => self.y -= cmd.length,
            Direction::Down => self.y += cmd.length,
            Direction::Forward => self.x += cmd.length,
        }
    }

    fn steer_problem2(&mut self, cmd: &Command) {
        match cmd.direction {
            Direction::Up => self.aim -= cmd.length,
            Direction::Down => self.aim += cmd.length,
            Direction::Forward => {
                self.x += cmd.length;
                self.y += cmd.length * self.aim;
            }
        }
    }
}

fn parse_command(cmd_string: String) -> Command {
    let elements = cmd_string.split(" ").collect::<Vec<&str>>();
    let dir = match elements[0] {
        "forward" => Direction::Forward,
        "down" => Direction::Down,
        "up" => Direction::Up,
        _ => panic!(),
    };
    let length = elements[1].parse::<i64>().unwrap();

    Command {
        direction: dir,
        length,
    }
}

fn problem1(data: &Vec<Command>, ship: &mut Ship) {
    for cmd in data.iter() {
        ship.steer_problem1(cmd)
    }

    println!("Problem 1 -> {}", ship.x * ship.y);
}

fn problem2(data: &Vec<Command>, ship: &mut Ship) {
    ship.reset();

    for cmd in data.iter() {
        ship.steer_problem2(cmd)
    }

    println!("Problem 2 -> {}", ship.x * ship.y);
}

fn main() {
    let data = parse_input_to_vec::<String>("input", "\n", basic_parser::<String>)
        .into_iter()
        .map(parse_command)
        .collect::<Vec<Command>>();

    let mut ship = Ship { x: 0, y: 0, aim: 0 };
    problem1(&data, &mut ship);
    problem2(&data, &mut ship);
}
