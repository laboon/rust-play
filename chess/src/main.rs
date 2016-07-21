mod turn;
mod board;
mod pieces;
mod rank;
mod file;

fn main() {
    let mut t = turn::Turn::White;
    let mut s = [[0i32; 8]; 8];
    let mut b = board::Board {
        squares: s,
        turn: t };
    b.setup();
    b.print();
}
