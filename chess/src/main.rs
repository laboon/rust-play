use std::io::prelude::*;
use std::io;

mod turn;
mod board;
mod pieces;
mod rank;
mod file;

fn get_move_from_console() -> String {
    
    print!(" > ");
    let mut choice = String::new();

    io::stdout().flush().expect("Could not flush stdout");
    io::stdin().read_line(&mut choice).expect("Could not read line");
    return choice;
}

#[allow(unused_assignments, unused_mut)]
fn main() {
    let mut t = turn::Turn::White;
    let mut s = [[0i32; 8]; 8];
    let mut b = board::Board {
        squares: s,
        turn: t };
    
    b.setup();
    loop {
        b.print();
        let move_string = get_move_from_console();
        b.make_move(move_string);
    }    
}
