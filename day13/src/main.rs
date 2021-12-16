#![feature(hash_drain_filter)]

type Points = std::collections::HashSet<(i32, i32)>;

struct FoldOperation {
    line_dim: char,
    idx: i32,
}

fn perform_fold(op: FoldOperation, points: &mut Points) {
    let moved_points: Vec<(i32, i32)> = points
        .drain_filter(|&(x, y)| {
            if op.line_dim == 'y' {
                y > op.idx
            } else {
                x > op.idx
            }
        })
        .map(|(x, y)| {
            if op.line_dim == 'y' {
                (x, 2 * op.idx - y)
            } else {
                (2 * op.idx - x, y)
            }
        })
        .collect();
    for point in moved_points {
        points.insert(point);
    }
}

fn main() {
    let matches = clap::App::new("transparent-origami")
        .version("0.1")
        .author("Jacob Lundgren")
        .args_from_usage("<FILENAME>    'The file containing the input'")
        .get_matches();

    let input = std::fs::read_to_string(matches.value_of("FILENAME").unwrap()).unwrap();
    let (points, folds) = input.split_once("\n\n").unwrap();
    let mut points: Points = points
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|line| -> (i32, i32) {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let mut folds = folds.split('\n').filter(|s| !s.is_empty()).map(|line| {
        let relevant = line.split(' ').nth(2).unwrap();
        let (line_dim, idx) = relevant.split_once('=').unwrap();
        FoldOperation {
            line_dim: line_dim.chars().next().unwrap(),
            idx: idx.parse().unwrap(),
        }
    });

    perform_fold(folds.next().unwrap(), &mut points);

    println!("After one fold, {} points are visible", points.len());

    for fold in folds {
        perform_fold(fold, &mut points);
    }

    let (max_x, max_y) = points.iter().fold((0, 0), |acc, &(x, y)| {
        (std::cmp::max(acc.0, x), std::cmp::max(acc.1, y))
    });

    for y in 0..=max_y {
        for x in 0..=max_x {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!("-");
            }
        }
        println!();
    }
}
