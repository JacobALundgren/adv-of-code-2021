fn check_is_low_point(coord: (usize, usize), map: &[Vec<u32>]) -> bool {
    let (col, row) = coord;
    let val = map[row][col];
    if (row != 0 && map[row - 1][col] <= val)
        || (row != map.len() - 1 && map[row + 1][col] <= val)
        || (col != 0 && map[row][col - 1] <= val)
        || (col != map[row].len() - 1 && map[row][col + 1] <= val)
    {
        return false;
    }
    true
}

fn find_size_of_basin(coord: (usize, usize), map: &mut [Vec<u32>]) -> u32 {
    let (col, row) = coord;
    if map[row][col] == 9 {
        return 0;
    }
    map[row][col] = 9;
    let mut sum = 1;
    if row != 0 {
        sum += find_size_of_basin((col, row - 1), map);
    }
    if row != map.len() - 1 {
        sum += find_size_of_basin((col, row + 1), map);
    }
    if col != 0 {
        sum += find_size_of_basin((col - 1, row), map);
    }
    if col != map[row].len() - 1 {
        sum += find_size_of_basin((col + 1, row), map);
    }

    sum
}

fn main() {
    let matches = clap::App::new("smoke-basin")
        .version("0.1")
        .author("Jacob Lundgren")
        .about("Analyzes the low points")
        .args_from_usage("<FILENAME>    'The file containing the input'")
        .get_matches();

    let input = std::fs::read_to_string(matches.value_of("FILENAME").unwrap()).unwrap();
    let mut map: Vec<Vec<u32>> = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut sum = 0;
    let mut basin_sizes = Vec::new();
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if check_is_low_point((col, row), &map) {
                sum += map[row][col] + 1;
                basin_sizes.push(find_size_of_basin((col, row), &mut map));
            }
        }
    }
    println!("Sum of low point risk values is: {}", sum);

    basin_sizes.sort_unstable();
    println!(
        "Three largest basin sizes: {:?}",
        basin_sizes.iter().rev().take(3).collect::<Vec<_>>()
    );
}
