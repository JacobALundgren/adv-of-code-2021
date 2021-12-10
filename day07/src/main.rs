fn part_1(mut nums: Vec<u32>) -> u32 {
    let median_idx = nums.len();
    let (lower, median, higher) = nums.select_nth_unstable(median_idx / 2);
    lower.iter().fold(0u32, |acc, &val| acc + *median - val)
        + higher.iter().fold(0u32, |acc, &val| acc + val - *median)
}

fn part_2(mut nums: Vec<u32>) -> u32 {
    nums.sort();
    if *nums.last().unwrap() == 0 {
        return 0;
    }
    let mut pos = 0;
    let mut idx = nums.iter().position(|&val| val > pos).unwrap() as i32;
    let mut change_per_step: i32 = -(nums.iter().sum::<u32>() as i32) + idx;
    let second_derivative: i32 = nums.len() as i32;

    while change_per_step < 0 {
        pos += 1;
        let next_idx = nums[(idx as usize)..].iter().position(|&val| val > pos).unwrap_or(nums.len() - idx as usize) as i32 + idx;
        change_per_step += second_derivative + (next_idx - idx);
        idx = next_idx;
    }

    nums.iter().fold(0i32, |acc, &val| acc + (1..=(pos as i32 - val as i32).abs()).sum::<i32>()) as u32
}

fn main() {
    let matches = clap::App::new("treachery-of-whales")
        .version("0.1")
        .author("Jacob Lundgren")
        .about("Finds the optimal alignment point and fuel consumption of crabs")
        .args_from_usage("<FILENAME>    'The file containing the input'")
        .args_from_usage("<PART>    'Whether to solve part 1 or 2'")
        .get_matches();

    let input = std::fs::read_to_string(matches.value_of("FILENAME").unwrap()).unwrap();
    let nums: Vec<_> = input.trim_end()
        .split(',')
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    let part: i32 = matches.value_of("PART").unwrap().parse().unwrap();
    let cost = if part == 1 {
        part_1(nums)
    } else {
        part_2(nums)
    };

    println!("Cost: {}", cost);
}
