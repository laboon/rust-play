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

    pub fn convert_to_array_spot(r: rank::Rank, f: file::File) -> (i32, i32) {
        let x = 8 - r;
        let mut y = -1;

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
            self.squares[1][j] = pieces::convert_piece_to_i32(pieces::Piece::Black_Pawn);
        }
        // Rooks
        self.squares[0][0] = pieces::convert_piece_to_i32(pieces::Piece::Black_Rook);
        self.squares[0][7] = pieces::convert_piece_to_i32(pieces::Piece::Black_Rook);
        // Knights
        self.squares[0][1] = pieces::convert_piece_to_i32(pieces::Piece::Black_Knight);
        self.squares[0][6] = pieces::convert_piece_to_i32(pieces::Piece::Black_Knight);
        // Bishops
        self.squares[0][2] = pieces::convert_piece_to_i32(pieces::Piece::Black_Bishop);
        self.squares[0][5] = pieces::convert_piece_to_i32(pieces::Piece::Black_Bishop);
        // Queen
        self.squares[0][3] = pieces::convert_piece_to_i32(pieces::Piece::Black_Queen);        
        // King
        self.squares[0][4] = pieces::convert_piece_to_i32(pieces::Piece::Black_King);

        // Set up white pieces
        // Pawns
        for j in 0..8 {
            self.squares[6][j] = pieces::convert_piece_to_i32(pieces::Piece::White_Pawn);
        }
        // Rooks
        self.squares[7][0] = pieces::convert_piece_to_i32(pieces::Piece::White_Rook);
        self.squares[7][7] = pieces::convert_piece_to_i32(pieces::Piece::White_Rook);
        // Knights
        self.squares[7][1] = pieces::convert_piece_to_i32(pieces::Piece::White_Knight);
        self.squares[7][6] = pieces::convert_piece_to_i32(pieces::Piece::White_Knight);
        // Bishops
        self.squares[7][2] = pieces::convert_piece_to_i32(pieces::Piece::White_Bishop);
        self.squares[7][5] = pieces::convert_piece_to_i32(pieces::Piece::White_Bishop);
        // Queen
        self.squares[7][3] = pieces::convert_piece_to_i32(pieces::Piece::White_Queen);        
        // King
        self.squares[7][4] = pieces::convert_piece_to_i32(pieces::Piece::White_King);

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

