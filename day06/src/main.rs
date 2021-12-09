fn main() {
    let matches = clap::App::new("lanternfish")
        .version("0.1")
        .author("Jacob Lundgren")
        .about("Finds the number of lantern fish")
        .args_from_usage("<FILENAME>    'The file containing the input'")
        .get_matches();

    let input = std::fs::read_to_string(matches.value_of("FILENAME").unwrap()).unwrap();
    let nums = input[0..input.len() - 1]
        .split(',')
        .map(|num| num.parse::<usize>().unwrap());
    let mut tracker = [0; 9];
    for i in nums {
        tracker[i] += 1;
    }
    for _ in 0..80 {
        tracker.rotate_left(1);
        tracker[6] += tracker[8];
    }
    println!(
        "Number of lantern fish after 80 days: {}",
        tracker.iter().sum::<i64>()
    );
    for _ in 0..(256 - 80) {
        tracker.rotate_left(1);
        tracker[6] += tracker[8];
    }
    println!(
        "Number of lantern fish after 256 days: {}",
        tracker.iter().sum::<i64>()
    );
}
