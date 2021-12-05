type ValueLocations = std::collections::HashMap<i32, (ndarray::Ix, ndarray::Ix)>;

struct Board {
    hits: ndarray::Array2<bool>,
    numbers: ValueLocations,
    won: bool,
}

impl Board {
    fn from_str(input: &str) -> Self {
        let numbers: ValueLocations = input
            .split('\n')
            .enumerate()
            .map(|(row_idx, row)| {
                row.split_ascii_whitespace()
                    .filter(|val| val.len() > 0)
                    .map(|val| val.parse::<i32>().unwrap())
                    .enumerate()
                    .map(move |(col_idx, num)| (num, (row_idx, col_idx)))
            })
            .flatten()
            .collect();
        let height = input.split('\n').filter(|row| row.len() > 0).count();
        let width = input
            .split_once('\n')
            .unwrap()
            .0
            .split_ascii_whitespace()
            .filter(|val| val.len() > 0)
            .count();
        Self {
            hits: ndarray::ArrayBase::from_elem((width, height), false),
            numbers,
            won: false,
        }
    }

    fn record_hit(&mut self, val: i32) -> bool {
        let index = self.numbers.remove(&val);
        index.map_or(false, |idx| {
            let location = self.hits.get_mut(idx).unwrap();
            *location = true;
            let won = self.hits.row(idx.0).into_iter().all(|hit| *hit)
                || self.hits.column(idx.1).into_iter().all(|hit| *hit);
            if won {
                self.won = true;
            }
            won
        })
    }
}

fn find_first_winning_board(
    draws: impl Iterator<Item = i32>,
    boards: &mut [Board],
) -> (i32, usize) {
    for draw in draws {
        for (idx, board) in boards.iter_mut().enumerate() {
            if board.record_hit(draw) {
                return (draw, idx);
            }
        }
    }
    panic!("No winning board found");
}

fn find_last_winning_board(draws: impl Iterator<Item = i32>, boards: &mut [Board]) -> (i32, usize) {
    let mut last_winning_draw = None;
    let mut last_winning_board = None;
    let mut remaining_boards = boards.len();
    for draw in draws {
        for (idx, board) in boards.iter_mut().enumerate() {
            if board.won {
                continue;
            }
            if board.record_hit(draw) {
                remaining_boards -= 1;
                last_winning_draw = Some(draw);
                last_winning_board = Some(idx);
                if remaining_boards == 0 {
                    return (draw, idx);
                }
            }
        }
    }
    (last_winning_draw.unwrap(), last_winning_board.unwrap())
}

fn main() {
    let matches = clap::App::new("bingo")
        .version("0.1")
        .author("Jacob Lundgren")
        .about("Finds the winning board")
        .args_from_usage("<FILENAME>    'The file containing the input'")
        .args_from_usage("<PART>    'Whether to solve part 1 or part 2'")
        .get_matches();

    let input = std::fs::read_to_string(matches.value_of("FILENAME").unwrap()).unwrap();
    let (draws, boards) = input.split_once("\n\n").unwrap();
    let draws = draws.split(',').map(|val| val.parse::<i32>().unwrap());
    let boards = boards.split("\n\n");
    let mut boards: Vec<Board> = boards.map(|input| Board::from_str(input)).collect();
    let part: i32 = matches.value_of("PART").unwrap().parse().unwrap();
    let (last_draw, winning_board) = if part == 1 {
        find_first_winning_board(draws, boards.as_mut_slice())
    } else {
        find_last_winning_board(draws, boards.as_mut_slice())
    };
    let remaining_sum: i32 = boards[winning_board].numbers.keys().sum();
    println!(
        "Last draw: {}, Sum of remaining numbers: {}",
        last_draw, remaining_sum
    );
}
