use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
    path::Path,
};

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
    let adjacent_differences = nums.iter().zip(nums.iter().skip(1)).map(|(l, r)| r - l);

    let increasing_count = adjacent_differences.filter(|val| val > &0).count();

    println!(
        "The number of instances where the depth increases is: {}",
        increasing_count
    );
}
