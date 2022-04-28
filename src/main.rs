use std::cmp::Ordering;
use std::collections::HashSet;

const DOT: char = '•';
use std::io::stdin;

struct Coordinate(i8, i8);

fn main() {
    let num_of_queens: usize;
    loop {
        println!("Please insert number in range: [4 - 13]. Over 13 will take too much time.");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        num_of_queens = match input.trim().parse() {
            Ok(num) => {
                if num < 4 || num > 27 {
                    continue;
                }
                num
            }
            Err(_) => continue,
        };
        break;
    }

    let mut all_cmbs: Vec<String> = Vec::new();
    let mut fundamental_cmbs: Vec<String> = Vec::new();

    let mut coordinates: Vec<Coordinate> = Vec::new();
    coordinates.push(Coordinate(0, 0));

    loop {
        if check_and_update_current_row(&mut coordinates, 0, num_of_queens) {
            if coordinates.len() == num_of_queens {
                check_and_add_new_cmb(
                    &coordinates,
                    &mut all_cmbs,
                    &mut fundamental_cmbs,
                    num_of_queens,
                );
            } else {
                continue;
            }
        }
        let job_finished = update_last_possible_row(&mut coordinates, num_of_queens);
        if job_finished {
            print_report(all_cmbs.len(), fundamental_cmbs, num_of_queens);
            break;
        }
    }
}

fn update_last_possible_row(coordinates: &mut Vec<Coordinate>, num_of_queens: usize) -> bool {
    loop {
        let start_col_idx = coordinates.last().unwrap().1 + 1;
        coordinates.pop();
        let did_update = check_and_update_current_row(coordinates, start_col_idx, num_of_queens);
        if did_update {
            return false;
        }
        if coordinates.len() == 0 {
            return true;
        }
    }
}

fn check_and_update_current_row(
    coordinates: &mut Vec<Coordinate>,
    start_col_idx: i8,
    num_of_queens: usize,
) -> bool {
    let mut used_cols = HashSet::new();

    for i in 0..coordinates.len() {
        used_cols.insert(coordinates[i].1);
    }

    if start_col_idx == num_of_queens as i8 {
        return false;
    }

    let row_idx = coordinates.len() as i8;
    for col_idx in start_col_idx..num_of_queens as i8 {
        if !used_cols.contains(&col_idx) && !check_diagonals(coordinates, col_idx, row_idx) {
            coordinates.push(Coordinate(row_idx, col_idx));
            return true;
        }
    }

    false
}

fn check_diagonals(coordinates: &Vec<Coordinate>, col_idx: i8, row_idx: i8) -> bool {
    check_desc_diagonal(coordinates, row_idx - col_idx)
        || check_asc_diagonal(coordinates, row_idx + col_idx)
}

fn check_desc_diagonal(coordinates: &Vec<Coordinate>, dif: i8) -> bool {
    coordinates.iter().any(|c| c.0 - c.1 == dif)
}

fn check_asc_diagonal(coordinates: &Vec<Coordinate>, sum: i8) -> bool {
    coordinates.iter().any(|c| c.0 + c.1 == sum)
}

fn check_and_add_new_cmb(
    coordinates: &Vec<Coordinate>,
    all_cmbs: &mut Vec<String>,
    fundamental_cmbs: &mut Vec<String>,
    num_of_queens: usize,
) {
    let combination = from_coordinates(coordinates);

    if !all_cmbs.contains(&combination) {
        fundamental_cmbs.push(combination);
        let current_cmb = fundamental_cmbs.last().unwrap();
        add_to_all_combinations(all_cmbs, current_cmb, coordinates, num_of_queens);
    }
}

fn add_to_all_combinations(
    all_cmbs: &mut Vec<String>,
    new_cmb: &String,
    coordinates: &Vec<Coordinate>,
    num_of_queens: usize,
) {
    all_cmbs.push(new_cmb.to_owned());
    let coordinates_rfl: Vec<Coordinate> = reflection(coordinates, num_of_queens);
    let new_cmb_rfl_str: String = from_coordinates(&coordinates_rfl);
    if !all_cmbs.contains(&new_cmb_rfl_str) {
        all_cmbs.push(new_cmb_rfl_str);
    }

    let coordinates_rot_90 = process_rot_by_90_and_rfl(all_cmbs, coordinates, num_of_queens);
    let coordinates_rot_180 =
        process_rot_by_90_and_rfl(all_cmbs, &coordinates_rot_90, num_of_queens);
    process_rot_by_90_and_rfl(all_cmbs, &coordinates_rot_180, num_of_queens);
}

fn process_rot_by_90_and_rfl(
    all_cmbs: &mut Vec<String>,
    coordinates: &Vec<Coordinate>,
    num_of_queens: usize,
) -> Vec<Coordinate> {
    let coordinates_rot_90: Vec<Coordinate> = rotate_90(coordinates, num_of_queens);
    let new_cmb_rot_90_str: String = from_coordinates(&coordinates_rot_90);
    let coordinates_rot_90_rfl: Vec<Coordinate> = reflection(&coordinates_rot_90, num_of_queens);
    let new_cmb_rot_90_rfl_str: String = from_coordinates(&coordinates_rot_90_rfl);

    if !all_cmbs.contains(&new_cmb_rot_90_str) {
        all_cmbs.push(new_cmb_rot_90_str);
    }

    if !all_cmbs.contains(&new_cmb_rot_90_rfl_str) {
        all_cmbs.push(new_cmb_rot_90_rfl_str);
    }

    coordinates_rot_90
}

fn reflection(coordinates: &Vec<Coordinate>, num_of_queens: usize) -> Vec<Coordinate> {
    let mut rfl_coordinates = vec![];
    for coordinate in coordinates {
        let new_col_idx = num_of_queens as i8 - coordinate.1 - 1;
        rfl_coordinates.push(Coordinate(coordinate.0, new_col_idx))
    }

    rfl_coordinates
}

fn rotate_90(coordinates: &Vec<Coordinate>, num_of_queens: usize) -> Vec<Coordinate> {
    let mut rfl_coordinates = vec![];
    for coordinate in coordinates {
        let new_col_idx = num_of_queens as i8 - coordinate.0 - 1;
        rfl_coordinates.push(Coordinate(coordinate.1, new_col_idx))
    }

    rfl_coordinates.sort_by(|a, b| {
        if a.0 < b.0 {
            Ordering::Less
        } else if a.0 == b.0 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });

    rfl_coordinates
}

fn print_report(all_cmbs_len: usize, fundamental_cmbs: Vec<String>, num_of_queens: usize) {
    println!("");
    for cmb_idx in 0..fundamental_cmbs.len() {
        let board_rows: Vec<Vec<char>> =
            create_board_represenation(fundamental_cmbs[cmb_idx].as_str(), num_of_queens);
        print_board(board_rows);
    }
    println!("Number of all combinations: {}", all_cmbs_len);
    println!(
        "Number of fundamental cominations: {}",
        fundamental_cmbs.len()
    );
}

fn create_board_represenation(combination: &str, num_of_queens: usize) -> Vec<Vec<char>> {
    let queen_coordinates: Vec<Coordinate> = combination
        .split(';')
        .map(|coordinates_as_str| {
            let x = coordinates_as_str.split(',').collect::<Vec<&str>>();
            Coordinate(x[0].parse::<i8>().unwrap(), x[1].parse::<i8>().unwrap())
        })
        .collect();

    let mut board = vec![vec!(' '; num_of_queens); num_of_queens];
    for coordinates in queen_coordinates {
        let row_idx = coordinates.0 as usize;
        let column_idx = coordinates.1 as usize;
        board[row_idx][column_idx] = DOT;
    }

    board
}

fn print_board(rows: Vec<Vec<char>>) {
    for row in rows {
        for cell in row {
            print!("|{}", cell);
        }
        print!("|\n");
    }
    println!("");
}

fn from_coordinates(coordinates: &Vec<Coordinate>) -> String {
    let mut combination = String::new();

    for coordinate in coordinates {
        combination.push_str(&coordinate.0.to_string());
        combination.push(',');
        combination.push_str(&coordinate.1.to_string());
        combination.push(';');
    }
    combination.pop();
    combination
}
