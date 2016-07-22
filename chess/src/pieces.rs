pub enum Piece {
    Empty,
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing
}

// TODO: fix this mess. Probably auto conversion?
// Or better yet arrays of structs instead of i32's?

pub fn convert_i32_to_piece(val: i32) -> Piece {
    match val {
        0 => return Piece::Empty,
        1 => return Piece::WhitePawn,
        2 => return Piece::WhiteKnight,
        3 => return Piece::WhiteBishop,
        4 => return Piece::WhiteRook,
        5 => return Piece::WhiteQueen,
        6 => return Piece::WhiteKing,
        7 => return Piece::BlackPawn,
        8 => return Piece::BlackKnight,
        9 => return Piece::BlackBishop,
        10 => return Piece::BlackRook,
        11 => return Piece::BlackQueen,
        12 => return Piece::BlackKing,
        _  => return Piece::Empty
    }
}

pub fn convert_piece_to_i32(p: Piece) -> i32 {
    match p {
        Piece::Empty        => return 0,
        Piece::WhitePawn   => return 1,
        Piece::WhiteKnight => return 2,
        Piece::WhiteBishop => return 3,
        Piece::WhiteRook   => return 4,
        Piece::WhiteQueen  => return 5,
        Piece::WhiteKing   => return 6,
        Piece::BlackPawn   => return 7,
        Piece::BlackKnight => return 8,
        Piece::BlackBishop => return 9,
        Piece::BlackRook   => return 10,
        Piece::BlackQueen  => return 11, 
        Piece::BlackKing   => return 12, 
    }
}

pub fn get_display(piece: Piece) -> char {
    match piece {
        Piece::Empty        => return '.',
        Piece::WhitePawn   => return 'P',
        Piece::WhiteKnight => return 'N',
        Piece::WhiteBishop => return 'B',
        Piece::WhiteRook   => return 'R',
        Piece::WhiteQueen  => return 'Q',
        Piece::WhiteKing   => return 'K',
        Piece::BlackPawn   => return 'p',
        Piece::BlackKnight => return 'n',
        Piece::BlackBishop => return 'b',
        Piece::BlackRook   => return 'r',
        Piece::BlackQueen  => return 'q',
        Piece::BlackKing   => return 'k',
    }
}

#[allow(dead_code)]
pub fn is_white_piece(piece: Piece) -> bool {
    match piece {
        Piece::WhitePawn   |
        Piece::WhiteKnight |
        Piece::WhiteBishop |
        Piece::WhiteRook   |
        Piece::WhiteQueen  |
        Piece::WhiteKing   => return true,
        _   => return false
    }
}

#[allow(dead_code)]
pub fn is_black_piece(piece: Piece) -> bool {
    match piece {
        Piece::BlackPawn   |
        Piece::BlackKnight |
        Piece::BlackBishop |
        Piece::BlackRook   |
        Piece::BlackQueen  |
        Piece::BlackKing   => return true,
        _   => return false
    }
}

