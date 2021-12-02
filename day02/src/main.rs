use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn parse_command(command: &str) -> (i32, i32) {
    let (direction, length) = command.split_once(' ').unwrap();
    let length = length.parse().unwrap();
    match direction {
        "forward" => (length, 0),
        "up" => (0, -length),
        "down" => (0, length),
        _ => panic!("Invalid direction"),
    }
}

fn main() {
    let matches = clap::App::new("dive")
        .version("0.1")
        .author("Jacob Lundgren")
        .about("Finds the final position of the submarine")
        .args_from_usage("<FILENAME>    'The file containing the input'")
        .args_from_usage("<PART>    'Whether to solve part 1 or part 2'")
        .get_matches();

    let path = Path::new(matches.value_of("FILENAME").unwrap());
    let display = path.display();

    let part: i32 = matches.value_of("PART").unwrap().parse().unwrap();

    let file = match File::open(&path) {
        Err(why) => panic!("Unable to open {}: {}", display, why),
        Ok(file) => BufReader::new(file),
    };

    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;
    for line in file.lines() {
        let travelled = parse_command(line.unwrap().as_str());
        if part == 1 {
            horizontal += travelled.0;
            vertical += travelled.1;
        } else if part == 2 {
            horizontal += travelled.0;
            vertical += travelled.0 * aim;
            aim += travelled.1;
        }
    }

    println!("horizontal: {}, vertical: {}", horizontal, vertical);
}
