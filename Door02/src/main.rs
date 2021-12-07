use std::str::FromStr;
use std::fmt;
use std::env;
use std::fs;

enum Direction {
    Forward,
    Down,
    Up
}

struct Command {
    direction : Direction,
    distance: u32
}

#[derive(Debug)]
struct ParseCommandError {
    cause : String
}

impl fmt::Display for ParseCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error Parsing Command: {}", self.cause)
    }
}

impl std::convert::From<std::num::ParseIntError> for ParseCommandError {
    fn from(err : std::num::ParseIntError) -> ParseCommandError {
        ParseCommandError{cause : err.to_string()}
    }
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts :Vec<&str> = s.split(' ').collect();

        let direction = match parts[0] {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            other => Err(ParseCommandError{cause: other.to_string()})
        }?;
        let distance = parts[1].parse::<u32>()?;

        Ok(Command{direction : direction, distance : distance})
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let use_aim : bool = args[2].parse::<bool>().unwrap_or(false);

    let contents = fs::read_to_string(filename)
        .expect("Something went terribly wrong while reading the file!");
    let lines = contents.split("\n")
                 .map(|line| line.parse::<Command>())
                 .filter(|line| line.is_ok())
                 .map(|line| line.unwrap());

    let mut depth : u32 = 0;
    let mut position : u32 = 0;
    let mut aim : i32 = 0;
    for Command{direction, distance} in lines {
        if use_aim {
            match direction {
                Direction::Up => { aim -= distance as i32 }
                Direction::Down => { aim += distance as i32 }
                Direction::Forward => {
                    position += distance;
                    depth = (depth as i32 + (distance as i32*aim)) as u32;
                }
            }
        } else {
            match direction {
                Direction::Forward => { position += distance }
                Direction::Up => { depth -= distance }
                Direction::Down => { depth += distance }
            }
        }
    }

    println!("Depth: {}\nPosition: {}\nProduct: {}", depth, position, depth*position);
}
