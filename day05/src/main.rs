type Point = (i32, i32);
fn parse_point(s: &str) -> Point {
    let (l, r) = s.split_once(',').unwrap();
    (l.parse().unwrap(), r.parse().unwrap())
}

type Line = (Point, Point);
fn parse_line(s: &str) -> Line {
    let (begin, end) = s.split_once(' ').unwrap();
    let end = end.strip_prefix("-> ").unwrap();
    (parse_point(begin), parse_point(end))
}

fn get_range(
    p1: i32,
    p2: i32,
) -> itertools::Either<impl Iterator<Item = i32>, impl Iterator<Item = i32>> {
    if p1 < p2 {
        itertools::Either::Left((p1..=p2).cycle())
    } else {
        itertools::Either::Right((p2..=p1).rev().cycle())
    }
}

fn get_points_on_line(line: &Line) -> impl Iterator<Item = Point> {
    let len =
        (std::cmp::max((line.1 .0 - line.0 .0).abs(), (line.1 .1 - line.0 .1).abs()) + 1) as usize;
    let xs = get_range(line.0 .0, line.1 .0);
    let ys = get_range(line.0 .1, line.1 .1);
    xs.zip(ys).take(len)
}

fn main() {
    let matches = clap::App::new("hydrothermal-venture")
        .version("0.1")
        .author("Jacob Lundgren")
        .about("Finds vent intersections")
        .args_from_usage("<FILENAME>    'The file containing the input'")
        .args_from_usage("<PART>   'Whether to solve part 1 or 2'")
        .get_matches();

    let part: i32 = matches.value_of("PART").unwrap().parse().unwrap();
    let part_predicate = if part == 1 {
        |line: &Line| line.0 .0 == line.1 .0 || line.0 .1 == line.1 .1
    } else {
        |_line: &Line| true
    };

    let input = std::fs::read_to_string(matches.value_of("FILENAME").unwrap()).unwrap();
    let vents = input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| parse_line(line))
        .filter(part_predicate);

    let mut point_occurrences = std::collections::HashMap::new();
    for vent in vents {
        for point in get_points_on_line(&vent) {
            let counter = point_occurrences.entry(point).or_insert(0);
            *counter += 1;
        }
    }

    let overlap_points = point_occurrences
        .iter()
        .filter(|(_, &value)| value > 1)
        .count();

    println!("Number of points with overlap: {}", overlap_points);
}
