#![feature(hash_drain_filter)]
use std::collections::HashMap;

fn char_occurrences(s: &str) -> HashMap<char, usize> {
    let mut ret = HashMap::new();
    for i in s.chars() {
        *ret.entry(i).or_default() += 1;
    }
    ret
}

fn char_pairs(s: &str) -> HashMap<(char, char), usize> {
    let mut ret = HashMap::new();
    for pair in s.as_bytes().windows(2) {
        *ret.entry((pair[0] as char, pair[1] as char)).or_default() += 1;
    }
    ret
}

fn run_iteration(
    rules: &HashMap<(char, char), char>,
    state: &mut HashMap<char, usize>,
    pairs: &mut HashMap<(char, char), usize>,
) {
    let consumed_pairs = pairs.drain_filter(|pair, _| rules.contains_key(pair));
    let mut generated_pairs = HashMap::<(char, char), usize>::new();
    for (pair, count) in consumed_pairs {
        let &produced = rules.get(&pair).unwrap();
        *state.entry(produced).or_default() += count;
        *generated_pairs.entry((pair.0, produced)).or_default() += count;
        *generated_pairs.entry((produced, pair.1)).or_default() += count;
    }
    for (pair, count) in generated_pairs.drain() {
        *pairs.entry(pair).or_default() += count;
    }
}

fn main() {
    let matches = clap::App::new("Extended Polymerization")
        .version("0.1")
        .author("Jacob Lundgren")
        .about("Simulates polymer growth")
        .args_from_usage("<FILENAME>    'The file containing the input'")
        .get_matches();

    let input = std::fs::read_to_string(matches.value_of("FILENAME").unwrap()).unwrap();
    let (initial, rules) = input.split_once("\n\n").unwrap();
    let initial = initial.trim();
    let rules = rules
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let constituents = (s.chars().next().unwrap(), s.chars().nth(1).unwrap());
            let produces = s.chars().nth(6).unwrap();
            (constituents, produces)
        })
        .collect::<HashMap<(char, char), char>>();

    let mut state = char_occurrences(initial);
    let mut pairs = char_pairs(initial);

    for _ in 0..10 {
        run_iteration(&rules, &mut state, &mut pairs);
    }
    let max = state.iter().max_by_key(|&(_, count)| count);
    let min = state.iter().min_by_key(|&(_, count)| count);
    println!(
        "Iter 10:\nMax count: {:?}\nMin count: {:?}",
        max.unwrap(),
        min.unwrap()
    );

    for _ in 0..(40 - 10) {
        run_iteration(&rules, &mut state, &mut pairs);
    }
    let max = state.iter().max_by_key(|&(_, count)| count);
    let min = state.iter().min_by_key(|&(_, count)| count);
    println!(
        "Iter 40:\nMax count: {:?}\nMin count: {:?}",
        max.unwrap(),
        min.unwrap()
    );
}
