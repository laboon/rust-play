use turn;
use pieces;
use rank;
use file;

// Board layout:
// V Array |    Rank V
// 0 r n b q k b n r 8
// 1 p p p p p p p p 7
// 2 _ _ _ _ _ _ _ _ 6
// 3 _ _ _ _ _ _ _ _ 5 
// 4 _ _ _ _ _ _ _ _ 4 
// 5 _ _ _ _ _ _ _ _ 3
// 6 P P P P P P P P 2 
// 7 R N B Q K B N R 1
//   0 1 2 3 4 5 6 7
//   a b c d e f g h

pub struct Board {
    pub squares: [[i32; 8]; 8],
    pub turn: turn::Turn
}

impl Board {

    #[allow(dead_code)]
    pub fn convert_to_array_spot(r: rank::Rank, f: file::File) -> (i32, i32) {
        let x = 8 - r;
        let y;

        match f {
            file::File::a => y = 0,
            file::File::b => y = 1,
            file::File::c => y = 2,
            file::File::d => y = 3,
            file::File::e => y = 4,
            file::File::f => y = 5,
            file::File::g => y = 6,
            file::File::h => y = 7
        }
        return (x, y)
    }

    pub fn make_move(&mut self, move_string: String) {
        println!("Moving: {}", move_string);
    }
    
    pub fn setup(&mut self) {
        println!("Setting up a new game");
        // Set up black pieces
        // Pawns
        for j in 0..8 {
            self.squares[1][j] = pieces::convert_piece_to_i32(pieces::Piece::BlackPawn);
        }
        // Rooks
        self.squares[0][0] = pieces::convert_piece_to_i32(pieces::Piece::BlackRook);
        self.squares[0][7] = pieces::convert_piece_to_i32(pieces::Piece::BlackRook);
        // Knights
        self.squares[0][1] = pieces::convert_piece_to_i32(pieces::Piece::BlackKnight);
        self.squares[0][6] = pieces::convert_piece_to_i32(pieces::Piece::BlackKnight);
        // Bishops
        self.squares[0][2] = pieces::convert_piece_to_i32(pieces::Piece::BlackBishop);
        self.squares[0][5] = pieces::convert_piece_to_i32(pieces::Piece::BlackBishop);
        // Queen
        self.squares[0][3] = pieces::convert_piece_to_i32(pieces::Piece::BlackQueen);        
        // King
        self.squares[0][4] = pieces::convert_piece_to_i32(pieces::Piece::BlackKing);

        // Set up white pieces
        // Pawns
        for j in 0..8 {
            self.squares[6][j] = pieces::convert_piece_to_i32(pieces::Piece::WhitePawn);
        }
        // Rooks
        self.squares[7][0] = pieces::convert_piece_to_i32(pieces::Piece::WhiteRook);
        self.squares[7][7] = pieces::convert_piece_to_i32(pieces::Piece::WhiteRook);
        // Knights
        self.squares[7][1] = pieces::convert_piece_to_i32(pieces::Piece::WhiteKnight);
        self.squares[7][6] = pieces::convert_piece_to_i32(pieces::Piece::WhiteKnight);
        // Bishops
        self.squares[7][2] = pieces::convert_piece_to_i32(pieces::Piece::WhiteBishop);
        self.squares[7][5] = pieces::convert_piece_to_i32(pieces::Piece::WhiteBishop);
        // Queen
        self.squares[7][3] = pieces::convert_piece_to_i32(pieces::Piece::WhiteQueen);        
        // King
        self.squares[7][4] = pieces::convert_piece_to_i32(pieces::Piece::WhiteKing);

    }

    pub fn print(&self) {
        for j in 0..8 {
            for k in 0..8 {
                let p = pieces::convert_i32_to_piece(self.squares[j][k]);
                print!("{} ", pieces::get_display(p));
            }
            println!("");
        }
    }

}

// TESTS

// #[test]
// fn convert_arr_a1() {
//     let mut t = turn::Turn::White;
//     let mut s = [[0i32; 8]; 8];
//     let mut b = board::Board {
//         squares: s,
//         turn: t };
//     let r = 1;
//     let f = file::File::a;
//     let (x, y) = b.convert_arr_a1(r, f);

// }
