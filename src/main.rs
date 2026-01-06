use rand::Rng;
use std::io;
use std::io::Write; //flush()
use std::mem;
use std::thread;
use std::time;
use colored_text::Colorize;
const BOARDSIZE: usize = 50;
const GENERATIONS: i32 = 500;
const WAITBTWNGENS: time::Duration = time::Duration::from_millis(50); 
const NBMETHOD: i8 = 1; //0 = seperate boards per colour, 1 = count other clrs in same cell as neigbours
const RULE: i8 = 1; // 0 = default, 1 = [spawn = 4; stay alive = 3-5]

#[derive(Debug, Clone)]
struct Cel {
    celchar: char,
    red: bool,
    green: bool,
    blue: bool
}

fn init() -> Vec<Vec<Cel>>{
    let mut rng = rand::rng();
    let mut board: Vec<Vec<Cel>> = Vec::new();
    for _row in 0..BOARDSIZE {
        let mut new_row: Vec<Cel> = Vec::new();
        for _cel in 0..BOARDSIZE {
            new_row.push(Cel{ celchar: '#', red: rng.random_bool(0.5), green: rng.random_bool(0.5), blue: rng.random_bool(0.5)});
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

    print!("{}", c.celchar.rgb(c.red as u8 * 255, c.green as u8 *255, c.blue as u8 *255));
}

fn draw_board(board: &Vec<Vec<Cel>>){
    //print!("{}[2J", 27 as char); // clear terminal screen
	print!("\x1Bc");
    for row in board.into_iter() {
        for col in row.into_iter(){
            draw_cel(&col);
        }
        print!("{}", "\n");
    }
    print!("{}", "\n");
    io::stdout().flush().unwrap();
}

fn in_bounds(row_idx: Option<usize>, col_idx: Option<usize>) -> bool {
    if row_idx == None || row_idx >= Some(BOARDSIZE) || col_idx == None || col_idx >= Some(BOARDSIZE) { return false; }
    true
}

fn count_neighbours(row_idx: usize, col_idx: usize, current_board: &Vec<Vec<Cel>>) -> (u8, u8, u8) {
    let mut total_nb: (u8, u8, u8) = (0,0,0);

    for row in [row_idx.checked_sub(1), Some(row_idx), row_idx.checked_add(1)] {
        for col in [col_idx.checked_sub(1), Some(col_idx), col_idx.checked_add(1)] {
            if in_bounds(row, col) { //not oob or None
                let row_uw = row.unwrap();
                let col_uw = col.unwrap();
                if !(row_uw == row_idx && col_uw == col_idx) { // not self
                    if current_board[row_uw][col_uw].red{total_nb.0 += 1;}
                    if current_board[row_uw][col_uw].green{total_nb.1 += 1;}
                    if current_board[row_uw][col_uw].blue{total_nb.2 += 1;}  
                } if NBMETHOD == 1 {
                    if current_board[row_uw][col_uw].red{total_nb.0 += current_board[row_uw][col_uw].green as u8 + current_board[row_uw][col_uw].blue as u8;}
                    if current_board[row_uw][col_uw].green{total_nb.1 += current_board[row_uw][col_uw].red as u8 + current_board[row_uw][col_uw].blue as u8;}
                    if current_board[row_uw][col_uw].blue{total_nb.2 += current_board[row_uw][col_uw].red as u8 + current_board[row_uw][col_uw].green as u8;}
                }
            }
        }
    } 
    total_nb
}

fn iterate_cel(nb: u8, alive: bool) -> bool{
    if RULE == 0 {
        if (!alive) && nb == 3 {            // spawn new life on dead cell
            return true;
        }
        if alive && (nb == 2 || nb == 3){   // continue life on living cell
            return true;
        }
    }
    if RULE == 1 {
        if (!alive) && nb == 4 {            // spawn new life on dead cell
            return true;
        }
        if (alive && nb >= 3 && nb <= 4){   // continue life on living cell
            return true;
        }
    }
        
    false
}


fn iterate_board(current_board: &Vec<Vec<Cel>>, next_board: &mut Vec<Vec<Cel>>){
    for row in 0..BOARDSIZE {
        for col in 0..BOARDSIZE{
            let nb: (u8, u8, u8) = count_neighbours(row, col, &current_board);
            next_board[row][col].red = iterate_cel(nb.0, current_board[row][col].red);
            next_board[row][col].green = iterate_cel(nb.1, current_board[row][col].green);
            next_board[row][col].blue = iterate_cel(nb.2, current_board[row][col].blue);
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
