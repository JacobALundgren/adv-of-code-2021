struct LineResult {
    final_stack: Vec<char>,
    error_char: Option<char>,
}

fn is_opening(c: char) -> bool {
    ['[', '(', '{', '<'].contains(&c)
}

fn is_match(open: char, close: char) -> bool {
    open == '[' && close == ']'
        || open == '(' && close == ')'
        || open == '{' && close == '}'
        || open == '<' && close == '>'
}

fn analyze_line(line: &str) -> LineResult {
    let mut stack = Vec::new();
    for c in line.chars() {
        if is_opening(c) {
            stack.push(c);
        } else {
            if stack.is_empty() || !is_match(*stack.last().unwrap(), c) {
                return LineResult {
                    final_stack: stack,
                    error_char: Some(c),
                };
            }
            stack.pop();
        }
    }
    LineResult {
        final_stack: stack,
        error_char: None,
    }
}

fn score_err_char(c: char) -> u32 {
    [(')', 3), (']', 57), ('}', 1197), ('>', 25137)]
        .iter()
        .find(|(close, _)| c == *close)
        .unwrap()
        .1
}

fn part_1<I: Iterator<Item = LineResult>>(results: I) {
    let score = results.fold(0, |acc, res| {
        acc + res.error_char.map_or(0, score_err_char)
    });
    println!("Score: {}", score);
}

fn score_missing_closer(c: char) -> u64 {
    [('(', 1), ('[', 2), ('{', 3), ('<', 4)]
        .iter()
        .find(|(open, _)| c == *open)
        .unwrap()
        .1
}

fn score_incomplete(stack: &[char]) -> u64 {
    stack
        .iter()
        .rfold(0, |acc, c| 5 * acc + score_missing_closer(*c))
}

fn part_2<I: Iterator<Item = LineResult>>(results: I) {
    let mut scores: Vec<u64> = results
        .filter(|res| res.error_char.is_none())
        .map(|res| score_incomplete(&res.final_stack))
        .collect();
    let len = scores.len();
    let median_score = *scores.select_nth_unstable(len / 2).1;
    println!("Median score: {}", median_score);
}

fn main() {
    let matches = clap::App::new("syntax-scoring")
        .version("0.1")
        .author("Jacob Lundgren")
        .about("Scores syntax errors")
        .args_from_usage("<FILENAME>    'The file containing the input'")
        .args_from_usage("<PART>    'Whether to solve part 1 or 2'")
        .get_matches();
    let part: i32 = matches.value_of("PART").unwrap().parse().unwrap();

    let input = std::fs::read_to_string(matches.value_of("FILENAME").unwrap()).unwrap();
    let results = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(analyze_line);

    if part == 1 {
        part_1(results);
    } else {
        part_2(results);
    }
}
