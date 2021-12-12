use std::collections::HashSet;

fn interpret_display_output(s: &str) -> Vec<u8> {
    let (numbers, output) = s.split_once(" | ").unwrap();
    let numbers: Vec<HashSet<char>> = numbers
        .split_ascii_whitespace()
        .map(|s| s.chars().collect())
        .collect();
    let one_segments = numbers
        .iter()
        .filter(|segments| segments.len() == 2)
        .next()
        .unwrap();
    let seven_segments = numbers
        .iter()
        .filter(|segments| segments.len() == 3)
        .next()
        .unwrap();
    let a_segment: char = *seven_segments.difference(one_segments).next().unwrap();
    let f_segment: char = *numbers
        .iter()
        .cloned()
        .filter(|segments| segments.len() == 6)
        .reduce(|a, b| &a & &b)
        .unwrap()
        .intersection(one_segments)
        .next()
        .unwrap();
    let c_segment: char = *one_segments.iter().find(|&&c| c != f_segment).unwrap();
    let four_segments = numbers
        .iter()
        .filter(|segments| segments.len() == 4)
        .next()
        .unwrap();
    let d_segment: char = *numbers
        .iter()
        .cloned()
        .filter(|segments| segments.len() == 5)
        .reduce(|a, b| &a & &b)
        .unwrap()
        .intersection(four_segments)
        .next()
        .unwrap();
    let g_segment: char = *numbers
        .iter()
        .cloned()
        .filter(|segments| segments.len() == 5)
        .reduce(|a, b| &a & &b)
        .unwrap()
        .iter()
        .find(|&&c| c != a_segment && c != d_segment)
        .unwrap();
    let b_segment: char = *numbers
        .iter()
        .cloned()
        .filter(|segments| segments.len() == 6)
        .reduce(|a, b| &a & &b)
        .unwrap()
        .iter()
        .find(|&&c| c != a_segment && c != f_segment && c != g_segment)
        .unwrap();
    let e_segment: char = ('a'..='g')
        .find(|&c| {
            c != a_segment
                && c != b_segment
                && c != c_segment
                && c != d_segment
                && c != f_segment
                && c != g_segment
        })
        .unwrap();
    let mut mappings = Vec::with_capacity(10);
    mappings.push((
        [
            a_segment, b_segment, c_segment, e_segment, f_segment, g_segment,
        ]
        .into_iter()
        .collect::<HashSet<char>>(),
        0,
    ));
    mappings.push((
        [c_segment, f_segment]
            .into_iter()
            .collect::<HashSet<char>>(),
        1,
    ));
    mappings.push((
        [a_segment, c_segment, d_segment, e_segment, g_segment]
            .into_iter()
            .collect::<HashSet<char>>(),
        2,
    ));
    mappings.push((
        [a_segment, c_segment, d_segment, f_segment, g_segment]
            .into_iter()
            .collect::<HashSet<char>>(),
        3,
    ));
    mappings.push((
        [b_segment, c_segment, d_segment, f_segment]
            .into_iter()
            .collect::<HashSet<char>>(),
        4,
    ));
    mappings.push((
        [a_segment, b_segment, d_segment, f_segment, g_segment]
            .into_iter()
            .collect::<HashSet<char>>(),
        5,
    ));
    mappings.push((
        [
            a_segment, b_segment, d_segment, e_segment, f_segment, g_segment,
        ]
        .into_iter()
        .collect::<HashSet<char>>(),
        6,
    ));
    mappings.push((
        [a_segment, c_segment, f_segment]
            .into_iter()
            .collect::<HashSet<char>>(),
        7,
    ));
    mappings.push((
        [
            a_segment, b_segment, c_segment, d_segment, e_segment, f_segment, g_segment,
        ]
        .into_iter()
        .collect::<HashSet<char>>(),
        8,
    ));
    mappings.push((
        [
            a_segment, b_segment, c_segment, d_segment, f_segment, g_segment,
        ]
        .into_iter()
        .collect::<HashSet<char>>(),
        9,
    ));

    output
        .split_ascii_whitespace()
        .map(|s| s.chars().collect())
        .map(|num: HashSet<char>| {
            mappings
                .iter()
                .find(|(segments, _)| num == *segments)
                .unwrap()
                .1
        })
        .collect()
}

fn main() {
    let matches = clap::App::new("seven-segment-search")
        .version("0.1")
        .author("Jacob Lundgren")
        .about("Interprets faulty display signals")
        .args_from_usage("<FILENAME>    'The file containing the input'")
        .get_matches();

    let input = std::fs::read_to_string(matches.value_of("FILENAME").unwrap()).unwrap();
    let displays: Vec<Vec<u8>> = input
        .split('\n')
        .filter(|s| s.len() > 0)
        .map(|s| interpret_display_output(s))
        .collect();
    let flattened = displays.iter().map(|vec| vec.into_iter()).flatten();

    let mut occurrences = [0; 10];
    for &num in flattened {
        occurrences[num as usize] += 1;
    }

    println!("{:?}", occurrences);

    let sum = displays.iter().fold(0, |acc, vec| {
        acc + 1000 * vec[0] as u32 + 100 * vec[1] as u32 + 10 * vec[2] as u32 + vec[3] as u32
    });
    println!("The sum is: {}", sum);
}
