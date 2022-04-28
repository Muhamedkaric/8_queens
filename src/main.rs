use std::cmp::Ordering;

const DOT: char = '•';
const NUM_OF_QUEENS: usize = 4;


struct Coordinate(i8, i8);

fn main() {
    /*
        Combinations that includes reflection and rotations;
        E.g. for n=4: all_cmbs = ["0,1;1,3;2,1;3,2", "0,1;1,3;2,1;3,2"];
        | |•| | |                | | |•| |
        | | | |•|  is reflexion  |•| | | |
        |•| | | |      of        | | | |•|
        | | |•| |                | |•| | |
    */
    let mut all_cmbs: Vec<String> = Vec::new();
    /*
        E.g. for n=4: fundamental_cmbs = ["0,1;1,3;2,1;3,2"];
        | |•| | |
        | | | |•|
        |•| | | |
        | | |•| |
    */ 
    let mut fundamental_cmbs: Vec<String> = Vec::new();


    let mut coordinates: Vec<Coordinate> = Vec::new();
    coordinates.push(Coordinate(0,0));
    
    loop {
        if check_and_insert_new_coordinate(&mut coordinates) {
           if coordinates.len() == NUM_OF_QUEENS - 1 {
               check_and_add_new_cmb(&coordinates, &mut all_cmbs, &mut fundamental_cmbs);
           } else {
               continue;
           }
        }
        let job_finished = update_last_possible_row(&mut coordinates);
        if job_finished {
            print_report(all_cmbs.len(), fundamental_cmbs);
            break;
        }
    }

}

fn update_last_possible_row(coordinates: &mut Vec<Coordinate>) -> bool {
    loop {
        let did_update = check_and_update_current_row(coordinates);
        if did_update {
            return false;
        }
        coordinates.pop();
        return coordinates.len() == 0;
    }
}


fn check_and_update_current_row(coordinates: &mut Vec<Coordinate>) -> bool {
    true
}

fn check_and_insert_new_coordinate(coordinates: &mut Vec<Coordinate>) -> bool {
    true
}

fn check_and_add_new_cmb(coordinates: &Vec<Coordinate>, all_cmbs: &mut Vec<String> , fundamental_cmbs: &mut Vec<String>) {
    let mut combination = from_coordinates(coordinates);

    if !all_cmbs.contains(&combination) {
        fundamental_cmbs.push(combination.clone());
        let current_cmb = fundamental_cmbs.last().unwrap();
        add_to_all_combinations(all_cmbs, current_cmb, coordinates);
    }
}

fn add_to_all_combinations(all_cmbs: &mut Vec<String>, new_cmb: &String, coordinates: &Vec<Coordinate>) {

    all_cmbs.push(new_cmb.to_owned());
    let coordinates_rfl: Vec<Coordinate>  = reflection(coordinates);
    let new_cmb_rfl_str: String = from_coordinates(&coordinates_rfl);
    if  !all_cmbs.contains(&new_cmb_rfl_str) {
        all_cmbs.push(new_cmb_rfl_str);
    }

    let coordinates_rot_90 = process_rot_by_90_and_rfl(all_cmbs, coordinates);
    let coordinates_rot_180 = process_rot_by_90_and_rfl(all_cmbs, &coordinates_rot_90);
     process_rot_by_90_and_rfl(all_cmbs, &coordinates_rot_180);

}

fn process_rot_by_90_and_rfl(all_cmbs: &mut Vec<String>, coordinates: &Vec<Coordinate> ) -> Vec<Coordinate> {
    let coordinates_rot_90: Vec<Coordinate> = rotate_90(coordinates);
    let new_cmb_rot_90_str: String = from_coordinates(&coordinates_rot_90);
    let coordinates_rot_90_rfl: Vec<Coordinate>  = reflection(&coordinates_rot_90);
    let new_cmb_rot_90_rfl_str: String = from_coordinates(&coordinates_rot_90_rfl);

    if  !all_cmbs.contains(&new_cmb_rot_90_str) {
        all_cmbs.push(new_cmb_rot_90_str);
    }

    if  !all_cmbs.contains(&new_cmb_rot_90_rfl_str) {
        all_cmbs.push(new_cmb_rot_90_rfl_str);
    }

    coordinates_rot_90
}

fn reflection (coordinates: &Vec<Coordinate>)-> Vec<Coordinate> {
    let mut rfl_coordinates = vec![];
    for coordinate in coordinates {
        let new_col_idx = NUM_OF_QUEENS as i8 - coordinate.1 - 1;
        rfl_coordinates.push(Coordinate(coordinate.0, new_col_idx))
    }
    
    rfl_coordinates
}

fn rotate_90(coordinates: &Vec<Coordinate>) -> Vec<Coordinate> {
    let mut rfl_coordinates = vec![];
    for coordinate in coordinates {
        let new_col_idx = NUM_OF_QUEENS as i8 - coordinate.0 - 1;
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


fn print_report(all_cmbs_len: usize , fundamental_cmbs:Vec<String>) {
    println!("Number of all combinations: {}", all_cmbs_len);
    println!("Number of fundamental cominations: {}", fundamental_cmbs.len());
    for cmb in fundamental_cmbs {
        let board_rows: Vec<Vec<char>> = create_board_represenation(cmb.as_str());
        print_board(board_rows);
    }
}

fn create_board_represenation(combination: &str) -> Vec<Vec<char>>{
    let queen_coordinates: Vec<Coordinate> = combination
    .split(';')
    .map(|coordinates_as_str| {
        let x = coordinates_as_str.split(',').collect::<Vec<&str>>();
        Coordinate(x[0].parse::<i8>().unwrap(), x[1].parse::<i8>().unwrap())
    }).collect();

    let mut board = vec!(vec!(' ';NUM_OF_QUEENS);NUM_OF_QUEENS);
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
    println!("\n");
}

fn from_coordinates(coordinates: &Vec<Coordinate>)-> String {
    let mut combination = String::new();

    for coordinate in coordinates {
        combination.push_str(&coordinate.0.to_string());
        combination.push(',');
        combination.push_str(&coordinate.1.to_string());
        combination.push(';');
    }
    combination
}



