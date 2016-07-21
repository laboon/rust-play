pub enum Piece {
    Empty,
    White_Pawn,
    White_Knight,
    White_Bishop,
    White_Rook,
    White_Queen,
    White_King,
    Black_Pawn,
    Black_Knight,
    Black_Bishop,
    Black_Rook,
    Black_Queen,
    Black_King
}

// TODO: fix this mess. Probably auto conversion?
// Or better yet arrays of structs instead of i32's?

pub fn convert_i32_to_piece(val: i32) -> Piece {
    match val {
        0 => return Piece::Empty,
        1 => return Piece::White_Pawn,
        2 => return Piece::White_Knight,
        3 => return Piece::White_Bishop,
        4 => return Piece::White_Rook,
        5 => return Piece::White_Queen,
        6 => return Piece::White_King,
        7 => return Piece::Black_Pawn,
        8 => return Piece::Black_Knight,
        9 => return Piece::Black_Bishop,
        10 => return Piece::Black_Rook,
        11 => return Piece::Black_Queen,
        12 => return Piece::Black_King,
        _  => return Piece::Empty
    }
}

pub fn convert_piece_to_i32(p: Piece) -> i32 {
    match p {
        Piece::Empty        => return 0,
        Piece::White_Pawn   => return 1,
        Piece::White_Knight => return 2,
        Piece::White_Bishop => return 3,
        Piece::White_Rook   => return 4,
        Piece::White_Queen  => return 5,
        Piece::White_King   => return 6,
        Piece::Black_Pawn   => return 7,
        Piece::Black_Knight => return 8,
        Piece::Black_Bishop => return 9,
        Piece::Black_Rook   => return 10,
        Piece::Black_Queen  => return 11, 
        Piece::Black_King   => return 12, 
    }
}

pub fn get_display(piece: Piece) -> char {
    match piece {
        Piece::Empty        => return '.',
        Piece::White_Pawn   => return 'P',
        Piece::White_Knight => return 'N',
        Piece::White_Bishop => return 'B',
        Piece::White_Rook   => return 'R',
        Piece::White_Queen  => return 'Q',
        Piece::White_King   => return 'K',
        Piece::Black_Pawn   => return 'p',
        Piece::Black_Knight => return 'n',
        Piece::Black_Bishop => return 'b',
        Piece::Black_Rook   => return 'r',
        Piece::Black_Queen  => return 'q',
        Piece::Black_King   => return 'k',
    }
}

pub fn is_white_piece(piece: Piece) -> bool {
    match piece {
        Piece::White_Pawn   |
        Piece::White_Knight |
        Piece::White_Bishop |
        Piece::White_Rook   |
        Piece::White_Queen  |
        Piece::White_King   => return true,
        _   => return false
    }
}

pub fn is_black_piece(piece: Piece) -> bool {
    match piece {
        Piece::Black_Pawn   |
        Piece::Black_Knight |
        Piece::Black_Bishop |
        Piece::Black_Rook   |
        Piece::Black_Queen  |
        Piece::Black_King   => return true,
        _   => return false
    }
}

