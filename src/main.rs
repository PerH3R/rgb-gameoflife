use rand::Rng;
use std::io;
use std::io::Write; // <--- bring flush() into scope
use std::mem;
use std::thread;
use std::time;
use colored_text::Colorize;
const BOARDSIZE: usize = 5;
const GENERATIONS: i32 = 250;
const WAITBTWNGENS: time::Duration = time::Duration::from_millis(100); 

#[derive(Debug, Clone)]
struct Cel {
    red: bool,
    green: bool,
    blue: bool
}

fn init() -> Vec<Vec<Cel>>{
    // let BOARDSIZE: usize = (x * y) as usize;
    let mut rng = rand::rng();
    let mut board: Vec<Vec<Cel>> = Vec::new();
    for _row in 0..BOARDSIZE {
        let mut new_row: Vec<Cel> = Vec::new();
        for _cel in 0..BOARDSIZE {
            new_row.push(Cel{ red: rng.random_bool(0.5), green: rng.random_bool(0.5), blue: rng.random_bool(0.5)});
        }
        board.push(new_row);
    }

    board //return
}

fn draw_cel(c: &Cel){
    if c.red == false && c.green == false && c.blue == false {
        print!(" ");
        return;
    }

    print!("{}", "#".rgb(c.red as u8 * 255, c.green as u8 *255, c.blue as u8 *255));
}

fn draw_board(board: &Vec<Vec<Cel>>){
    for row in board.into_iter() {
        for col in row.into_iter(){
            draw_cel(&col);
        }
        print!("{}", "\n");
    }
    print!("{}", "\n");
    io::stdout().flush().unwrap();
}

fn count_neighbours(row_idx: usize, col_idx: usize, current_board: &Vec<Vec<Cel>>) -> (u8, u8, u8) {
    return (2,2,2)
}

fn iterate_cel(nb: u8) -> bool{
    if nb <= 1 || nb >= 4{
        return false;
    }
    true
}


fn iterate_board(current_board: &Vec<Vec<Cel>>, next_board: &mut Vec<Vec<Cel>>){
    for row in 0..current_board.len() {
        for col in 0..current_board.len(){
            let nb: (u8, u8, u8) = count_neighbours(row, col, &current_board);
            next_board[row][col].red = iterate_cel(nb.0);
            next_board[row][col].green = iterate_cel(nb.1);
            next_board[row][col].blue = iterate_cel(nb.2);
        }
    }
}


fn main() {
    let mut current_board: Vec<Vec<Cel>> = init();
    let mut next_board: Vec<Vec<Cel>> = init();
    draw_board(&current_board);
    for _g in 0..GENERATIONS {
        iterate_board(&current_board, &mut next_board);
        draw_board(&next_board);
        mem::swap(&mut current_board, &mut next_board);
        thread::sleep(WAITBTWNGENS);
    }
    
}
