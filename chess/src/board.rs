use turn;
use pieces;

// Board layout:
// 7 r n b q k b n r
// 6 p p p p p p p p
// 5 _ _ _ _ _ _ _ _
// 4 _ _ _ _ _ _ _ _
// 3 _ _ _ _ _ _ _ _
// 2 _ _ _ _ _ _ _ _
// 1 P P P P P P P P
// 0 R N B Q K B N R
//   0 1 2 3 4 5 6 7
//   a b c d e f g h

pub struct Board {
    squares: [[i32; 8]; 8],
    turn: turn::Turn
}

impl Board {
    fn setup(&self) {
        self.squares
        println!("Setting up a new game");
    }
}
