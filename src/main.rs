use std::io::{self, BufRead};

fn print_sudoku(sudoku: &Vec<Vec<u32>>) {
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

    print_sudoku(&sudoku);
    Ok(())
}
