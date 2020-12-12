extern crate utils;

use utils::parse_input_to_vec;

enum Operation { N, S, E, W, L, R, F }

struct NavigationInstruction {
    op: Operation,
    value: i32,
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Mul<Point> for i32 {
    type Output = Point;

    fn mul(self, _rhs: Point) -> Point {
        Point {
            x: self * _rhs.x,
            y: self * _rhs.y,
        }
    }
}


impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, _rhs: Self) {
        *self = Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, _rhs: Self) {
        *self = Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

#[derive(Debug)]
enum Direction { N, S, E, W }

#[derive(Debug)]
struct Ship {
    dir: Direction,
    position: Point,
    waypoint: Point,
}

fn char_to_operation(c: Option<char>) -> Option<Operation> {
    match c.unwrap() {
        'N' => Some(Operation::N),
        'S' => Some(Operation::S),
        'E' => Some(Operation::E),
        'W' => Some(Operation::W),
        'L' => Some(Operation::L),
        'R' => Some(Operation::R),
        'F' => Some(Operation::F),
        _ => None
    }
}

fn navigation_instructions_parser(line: &str) -> NavigationInstruction {
    let mut chars = line.chars();

    let op = char_to_operation(chars.next()).unwrap();
    let value = chars
        .collect::<String>()
        .parse::<i32>()
        .unwrap();

    NavigationInstruction {
        op,
        value,
    }
}

fn turn_left(ship: &mut Ship, value: i32) {
    if value == 0 {
        return;
    }

    match ship.dir {
        Direction::N => ship.dir = Direction::W,
        Direction::W => ship.dir = Direction::S,
        Direction::S => ship.dir = Direction::E,
        Direction::E => ship.dir = Direction::N,
    }

    turn_left(ship, value - 90);
}

fn turn_right(ship: &mut Ship, value: i32) {
    if value == 0 {
        return;
    }

    match ship.dir {
        Direction::S => ship.dir = Direction::W,
        Direction::E => ship.dir = Direction::S,
        Direction::N => ship.dir = Direction::E,
        Direction::W => ship.dir = Direction::N,
    }

    turn_right(ship, value - 90);
}

fn turn_waypoint_left(ship: &mut Ship, value: i32) {
    if value == 0 {
        return;
    }

    ship.waypoint = Point {
        x: ship.waypoint.y,
        y: -ship.waypoint.x,
    };

    turn_waypoint_left(ship, value - 90);
}

fn turn_waypoint_right(ship: &mut Ship, value: i32) {
    if value == 0 {
        return;
    }

    ship.waypoint = Point {
        x: -ship.waypoint.y,
        y: ship.waypoint.x,
    };

    turn_waypoint_right(ship, value - 90);
}

fn move_ship(ship: &mut Ship, instruction: &NavigationInstruction) {
    match instruction.op {
        Operation::N => ship.position.x += instruction.value,
        Operation::S => ship.position.x -= instruction.value,
        Operation::E => ship.position.y += instruction.value,
        Operation::W => ship.position.y -= instruction.value,
        Operation::L => turn_left(ship, instruction.value),
        Operation::R => turn_right(ship, instruction.value),
        Operation::F => match ship.dir {
            Direction::N => ship.position.x += instruction.value,
            Direction::E => ship.position.y += instruction.value,
            Direction::S => ship.position.x -= instruction.value,
            Direction::W => ship.position.y -= instruction.value,
        },
    };
}

fn move_ship_waypoint(ship: &mut Ship, instruction: &NavigationInstruction) {
    match instruction.op {
        Operation::N => ship.waypoint.x += instruction.value,
        Operation::S => ship.waypoint.x -= instruction.value,
        Operation::E => ship.waypoint.y += instruction.value,
        Operation::W => ship.waypoint.y -= instruction.value,
        Operation::L => turn_waypoint_left(ship, instruction.value),
        Operation::R => turn_waypoint_right(ship, instruction.value),
        Operation::F => match ship.dir {
            Direction::N => ship.position += instruction.value * ship.waypoint,
            Direction::E => ship.position += instruction.value * ship.waypoint,
            Direction::S => ship.position -= instruction.value * ship.waypoint,
            Direction::W => ship.position -= instruction.value * ship.waypoint,
        },
    };
}

fn manhattan_distance(ship: &Ship) -> i32 {
    ship.position.x.abs() + ship.position.y.abs()
}

fn problem1(instructions: &Vec<NavigationInstruction>) {
    let mut ship = Ship {
        dir: Direction::E,
        position: Point { x: 0, y: 0 },
        waypoint: Point { x: 0, y: 0 },
    };

    for instruction in instructions.iter() {
        move_ship(&mut ship, &instruction);
    }
    let distance = manhattan_distance(&ship);
    println!("Problem 1 -> {}", distance);
}

fn problem2(instructions: &Vec<NavigationInstruction>) {
    let mut ship = Ship {
        dir: Direction::E,
        position: Point { x: 0, y: 0 },
        waypoint: Point { x: 1, y: 10 },
    };

    for instruction in instructions.iter() {
        move_ship_waypoint(&mut ship, &instruction);
    }
    let distance = manhattan_distance(&ship);
    println!("Problem 1 -> {}", distance);
}

fn main() {
    let navigation_instructions = parse_input_to_vec(
        "input",
        "\n",
        navigation_instructions_parser,
    );

    problem1(&navigation_instructions);
    problem2(&navigation_instructions);
}
