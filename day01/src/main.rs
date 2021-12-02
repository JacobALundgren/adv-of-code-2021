use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
    path::Path,
};

fn num_increases_windowed(nums: &[i32], window_size: usize) -> usize {
    let windows = nums.windows(window_size);
    let sums: Vec<i32> = windows.map(|window| window.iter().sum()).collect();
    let adjacent_differences = sums.iter().zip(sums.iter().skip(1)).map(|(l, r)| r - l);
    adjacent_differences.filter(|val| val > &0).count()
}

fn main() {
    let matches = clap::App::new("sonar-sweep")
        .version("0.1")
        .author("Jacob Lundgren")
        .about("Finds the number of instances of depth increasing")
        .args_from_usage("<FILENAME>     'The file containing the input'")
        .get_matches();

    let path = Path::new(matches.value_of("FILENAME").unwrap());
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Unable to open {}: {}", display, why),
        Ok(file) => BufReader::new(file),
    };

    let nums: Result<Vec<i32>, Error> = file
        .lines()
        .map(|line| {
            line.and_then(|val| {
                val.parse()
                    .map_err(|e| Error::new(ErrorKind::InvalidData, e))
            })
        })
        .collect();

    let nums = nums.unwrap();

    // First problem
    println!(
        "The number of instances where the depth increases is: {}",
        num_increases_windowed(&nums, 1)
    );

    // Second problem
    println!(
        "The number of instances where the windowed total depth increases is: {}",
        num_increases_windowed(&nums, 3)
    );
}
