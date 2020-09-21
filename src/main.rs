use std::io::{self, BufRead};

type Sudoku = Vec<Vec<u32>>;

fn check_row(sudoku: &Sudoku, number: u32, row: u32, col: u32) -> bool {
    // if all values in sudoku[row][1..9] are not number
    // excluding sudoku[row][col] then return true
    for col_check in 0..9 {
        if col_check != col && sudoku[row as usize][col_check as usize] == number {
            return false;
        }
    }
    return true;
}

fn check_col(sudoku: &Sudoku, number: u32, row: u32, col: u32) -> bool {
    // if all values in sudoku[0..8][col] are not number
    // excluding sudoku[row][col] then return true
    for row_check in 0..9 {
        if row_check != row && sudoku[row_check as usize][col as usize] == number {
            return false;
        }
    }
    return true;
}

fn check_box(sudoku: &Sudoku, number: u32, row: u32, col: u32) -> bool {
    // This is a slightlier trickier one.
    // In order to iterate through box, we need to get the top left corner
    let top_row_of_box = row - row % 3;
    let left_col_of_box = col - col % 3;
    for box_x in 0..3 {
        for box_y in 0..3 {
            let check_row = top_row_of_box + box_x;
            let check_col = left_col_of_box + box_y;
            if check_row != row && check_col != col && number == sudoku[check_row as usize][check_col as usize] {
                return false;
            }
        }
    }
    return true;
}

fn can_assign_num_to_square(sudoku: &Sudoku, number: u32, row: u32, col: u32) -> bool {
    return check_box(&sudoku, number, row, col) && check_col(&sudoku, number, row, col) && check_row(&sudoku, number, row, col);
}

fn solve_sudoku(sudoku: &mut Sudoku) -> bool {
    let blank_square = find_blank(&sudoku);
    while blank_square.is_some() {
        let (r, c) = blank_square.unwrap();
        for x in 1..10 {
            if can_assign_num_to_square(&sudoku, x, r, c) {
                sudoku[r as usize][c as usize] = x;
                if solve_sudoku(sudoku) {
                    return true;
                }
                // Invalid, set it back to blank and continue
                sudoku[r as usize][c as usize] = 0;
            }
        }
        return false;
    }
    return true;
}

fn find_blank(sudoku: &Sudoku) -> Option<(u32, u32)> {
    for (row_ind, x) in sudoku.iter().enumerate() {
        for (col_ind, value) in x.iter().enumerate() {
            if *value == 0 {
                return Some((row_ind as u32, col_ind as u32));
            }
        }
    }
    None
}

fn print_sudoku(sudoku: &Sudoku) {
    println!("------------");
    let mut row_count = 0;
    for x in sudoku.iter() {
        if row_count > 2 {
            println!("------------");
            row_count = 0;
        }
        let mut col_count = 0;
        for value in x.iter() {
            if col_count > 2 {
                print!("|");
                col_count = 0;
            }
            if *value == 0 {
                print!(" ");
            } else {
                print!("{}", value);
            }
            col_count += 1;
        }
        println!("");
        row_count += 1;
    }
    println!("------------");
}

fn main() -> io::Result<()> {

    let buffer = io::stdin().lock().lines().next().unwrap().unwrap();

    if buffer.len() < 81 {
        println!("Not enough information");
        return Ok(());
    }

    let mut sudoku = vec![vec![0; 9]; 9]; 

    // Let's fill our sudoku in
    for (ind, digit) in buffer.chars().enumerate() {
        sudoku[ind / 9][ind % 9] = digit.to_digit(10).unwrap();
    }

    if solve_sudoku(&mut sudoku) {
        print_sudoku(&sudoku);
    } else {
        println!("Couldn't solve sudoku! :(");
    }
    Ok(())
}
