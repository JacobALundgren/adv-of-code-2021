type EdgeMap = std::collections::HashMap<String, Vec<String>>;

fn is_valid_visit_part_1(target: &str, visited: &[String]) -> bool {
    target.chars().next().unwrap().is_ascii_uppercase() || !visited.iter().any(|s| s == target)
}

fn is_valid_visit_part_2(target: &str, visited: &[String]) -> bool {
    if target == "start" {
        return false;
    }
    if target.chars().next().unwrap().is_ascii_uppercase() {
        return true;
    }
    if !visited.iter().any(|s| s == target) {
        return true;
    }
    for (idx, s) in visited
        .iter()
        .enumerate()
        .filter(|(_, s)| s.chars().next().unwrap().is_ascii_lowercase())
    {
        if visited[idx + 1..].contains(s) {
            return false;
        }
    }
    true
}

fn count_paths_to_end<P>(
    visited: &mut Vec<String>,
    edges: &EdgeMap,
    validity_predicate: &P,
) -> usize
where
    P: Fn(&str, &[String]) -> bool,
{
    if visited.last().unwrap() == "end" {
        return 1;
    }
    let mut num_paths = 0;
    let curr_edges = edges.get(visited.last().unwrap()).unwrap();
    for edge in curr_edges.iter() {
        if !validity_predicate(edge, visited) {
            continue;
        }
        visited.push(edge.clone());
        num_paths += count_paths_to_end(visited, edges, validity_predicate);
        visited.pop();
    }
    num_paths
}

fn main() {
    let matches = clap::App::new("passage-pathing")
        .version("0.1")
        .author("Jacob Lundgren")
        .args_from_usage("<FILENAME>    'The file containing the input'")
        .args_from_usage("<PART>    'Whether to solve part 1 or 2'")
        .get_matches();
    let part: i32 = matches.value_of("PART").unwrap().parse().unwrap();

    let validity_predicate = if part == 1 {
        is_valid_visit_part_1
    } else {
        is_valid_visit_part_2
    };

    let input = std::fs::read_to_string(matches.value_of("FILENAME").unwrap()).unwrap();
    let edges =
        input
            .split('\n')
            .filter(|s| !s.is_empty())
            .fold(EdgeMap::new(), |mut acc, line| {
                let (l, r) = line.split_once('-').unwrap();
                acc.entry(l.to_owned()).or_default().push(r.to_owned());
                acc.entry(r.to_owned()).or_default().push(l.to_owned());
                acc
            });
    let num_paths = count_paths_to_end(&mut vec!["start".to_owned()], &edges, &validity_predicate);
    println!("The number of paths is: {}", num_paths);
}
