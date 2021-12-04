fn main() {
    let matches = clap::App::new("binary-diagnostic")
        .version("0.1")
        .author("Jacob Lundgren")
        .about("Finds the power consumption of the submarine")
        .args_from_usage("<FILENAME>    'The file containing the input'")
        .get_matches();

    let input = std::fs::read_to_string(matches.value_of("FILENAME").unwrap()).unwrap();
    let bits = input.as_str().chars().position(|c| c == '\n').unwrap();
    let lines = input.split('\n');
    let nums: Vec<u32> = lines
        .filter(|line| line.len() > 0)
        .map(|line| u32::from_str_radix(line, 2).unwrap()).collect();
    let mut totals = vec![0; bits];

    for num in nums.iter() {
        let mut num = *num;
        let mut total_iter = totals.iter_mut().rev();
        while num > 0 {
            let curr = total_iter.next().unwrap();
            *curr += ((num & 0x1) == 0x1) as u32;
            num >>= 1;
        }
    }
    
    let mut gamma_rate = 0;
    for total in totals.iter() {
        gamma_rate *= 2;
        gamma_rate += (*total >= (nums.len() as u32 / 2)) as u32;
    }
    let epsilon_rate = 2u32.pow(bits as u32) - 1 - gamma_rate;

    let o2_generator_rating = {
        let mut candidates = nums.clone();
        let mut bitmask = 1 << (bits - 1);
        while candidates.len() > 1 {
            let count_ones = candidates.iter().fold(0, |acc, &num| acc + ((num & bitmask) > 0) as usize);
            let desired = if 2 * count_ones >= candidates.len() {
                bitmask
            } else {
                0
            };
            candidates.retain(|&val| (val & bitmask) == desired);
            bitmask >>= 1;
        }
        candidates[0]
    };
    let co2_scrubber_rating = {
        let mut candidates = nums.clone();
        let mut bitmask = 1 << (bits - 1);
        while candidates.len() > 1 {
            let count_ones = candidates.iter().fold(0, |acc, &num| acc + ((num & bitmask) > 0) as usize);
            let desired = if 2 * count_ones < candidates.len() {
                bitmask
            } else {
                0
            };
            candidates.retain(|&val| (val & bitmask) == desired);
            bitmask >>= 1;
        }
        candidates[0]
    };

    println!("gamma: {}, epsilon: {}", gamma_rate, epsilon_rate);
    println!("o2_generator: {}, co2_scrubber: {}", o2_generator_rating, co2_scrubber_rating);
}
