use std::{fs::File, io::Read};

/// Read the instruction file into a string
fn read_file(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut data = String::new();
    let mut file = File::open(filename)?;
    file.read_to_string(&mut data)?;

    Ok(data)
}

/// The only directions our sub can travel
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Forward,
}

/// An instruction for our sub to follow
#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: i32,
}

#[derive(Debug)]
struct Location {
    pub x: i32,
    pub y: i32,
    pub aim: i32,
}

impl Location {
    pub fn adjust_location_p1(&mut self, inst: &Instruction) {
        match inst.direction {
            // Our depth decreases if we go up, increases if we go down
            Direction::Up => self.y -= inst.distance,
            Direction::Down => self.y += inst.distance,
            Direction::Forward => self.x += inst.distance,
        }
    }
    pub fn adjust_location_p2(&mut self, inst: &Instruction) {
        match inst.direction {
            // Our aim decreases if we go up, increases if we go down
            Direction::Up => self.aim -= inst.distance,
            Direction::Down => self.aim += inst.distance,
            // our depth and distance are affected by moving forward, based on our aim
            Direction::Forward => {
                self.x += inst.distance;
                self.y += self.aim * inst.distance;
            }
        }
    }
}

/// Parse an instruction from a line in the file
fn parse_instruction(line: String) -> Option<Instruction> {
    let dir: Direction;
    let split: Vec<&str> = line.split_whitespace().collect();
    let distance: i32 = split[1].parse().unwrap();

    match split[0] {
        "up" => dir = Direction::Up,
        "down" => dir = Direction::Down,
        "forward" => dir = Direction::Forward,
        _ => return None,
    }

    Some(Instruction {
        direction: dir,
        distance: distance,
    })
}

fn part_one(directions_file: &str) {
    let mut pos = Location { x: 0, y: 0, aim: 0 };

    // Read our instructions
    read_file(directions_file)
        .unwrap()
        .as_str()
        // line by line
        .lines()
        // translate the instruction string into something we understand
        .map(|x| parse_instruction(x.to_string()).unwrap())
        // change our position based on the instruction
        .for_each(|i| pos.adjust_location_p1(&i));

    println!("Part One: {}", pos.x * pos.y);
}

fn part_two(directions_file: &str) {
    let mut loc = Location { x: 0, y: 0, aim: 0 };

    read_file(directions_file)
        .unwrap()
        .as_str()
        // line by line
        .lines()
        // translate the instruction string into something we understand
        .map(|x| parse_instruction(x.to_string()).unwrap())
        // change our position based on the instruction
        .for_each(|x| loc.adjust_location_p2(&x));

    println!("Part Two: {}", loc.x * loc.y);
}

fn main() {
    part_one("resources/instructions.txt");
    part_two("resources/instructions.txt");
}
