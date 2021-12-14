fn react_to_flash(coords: (i32, i32), map: &mut [Vec<u32>]) -> u32 {
    let (col, row) = coords;
    if col < 0 || col >= map[0].len() as i32 || row < 0 || row >= map.len() as i32 {
        return 0;
    }
    let flashes = {
        let entry = &mut map[row as usize][col as usize];
        *entry += 1;
        if *entry == 10 || *entry == 11 {
            *entry += 1;
            true
        } else {
            false
        }
    };

    if flashes {
        let mut total_flashes = 1;
        total_flashes += react_to_flash((col - 1, row - 1), map);
        total_flashes += react_to_flash((col, row - 1), map);
        total_flashes += react_to_flash((col + 1, row - 1), map);
        total_flashes += react_to_flash((col - 1, row), map);
        total_flashes += react_to_flash((col, row), map);
        total_flashes += react_to_flash((col + 1, row), map);
        total_flashes += react_to_flash((col - 1, row + 1), map);
        total_flashes += react_to_flash((col, row + 1), map);
        total_flashes += react_to_flash((col + 1, row + 1), map);
        total_flashes
    } else {
        0
    }
}

fn step(map: &mut [Vec<u32>]) -> u32 {
    for row in map.iter_mut() {
        for i in row {
            *i += 1;
        }
    }
    let mut total_flashes = 0;

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 10 {
                total_flashes += react_to_flash((col as i32, row as i32), map);
            }
        }
    }

    for row in map.iter_mut() {
        for i in row {
            if *i > 9 {
                *i = 0;
            }
        }
    }

    total_flashes
}

fn main() {
    let matches = clap::App::new("dumbo-octopus")
        .version("0.1")
        .author("Jacob Lundgren")
        .about("Simulates octopodes flashing")
        .args_from_usage("<FILENAME>    'The file containing the input'")
        .get_matches();

    let input = std::fs::read_to_string(matches.value_of("FILENAME").unwrap()).unwrap();
    let mut map: Vec<Vec<u32>> = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut total_flashes = 0;
    let mut first_all_flash_iteration = None;
    for i in 0..100 {
        let curr_flashes = step(&mut map);
        total_flashes += curr_flashes;
        if curr_flashes == (map.len() * map[0].len()) as u32 {
            first_all_flash_iteration = Some(i + 1);
        }
    }

    let mut i = 100;
    while first_all_flash_iteration.is_none() {
        let curr_flashes = step(&mut map);
        if curr_flashes == (map.len() * map[0].len()) as u32 {
            first_all_flash_iteration = Some(i + 1);
        }
        i += 1;
    }

    println!(
        "Total number of flashes after 100 iterations: {}",
        total_flashes
    );
    println!(
        "First iteration where all octopodes flash: {}",
        first_all_flash_iteration.unwrap()
    );
}
