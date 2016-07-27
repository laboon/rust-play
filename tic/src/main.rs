// Tic-Tac-Schmoe
// See CS1632_Summer repository for rules

extern crate rand;

// Use for I/O
use std::io;
use std::io::Error;

// use for getting random numbers - relies on extern crate rand
use rand::Rng;

// Square is the enum indicating what is located on a position
// of the tic-tac-schmoe board
// The #[derive(Copy, Clone)] allows this to make copies so that
// we can initialize an array with this.  Ordinarily it would
// not be necessary
#[derive(Copy, Clone)]
enum Square {
    X,
    O,
    Empty
}

// Player can be one of first player (X)
// or second player (O)

enum Player {
    X,
    O
}

// Possible final statuses of the game - either one player
// wins or there is a tie.

enum GameStatus {
    Player1Win,
    Player2Win,
    Tie
}

// This is basically an alias for the type "3 x 3 array of Squares"
// You can use types to define things more specifically than
// just generic types (e.g., perhaps some 

type Board = [[Square; 3]; 3];

// Get a name from the user and return it wrapped in a Result.
// If we can't accept the name for some reason (should not
// often happen in the console), it will return a result
// wrapped in an Error.  Otherwise, it will return a result
// wrapped in an Ok.

fn get_name() -> Result<String, Error> {
    let mut to_return = String::new();
    io::stdin().read_line(&mut to_return).expect("FAIL");
    Ok(to_return)
}

// Print the board to the console.
// No other side effects.

fn print_board(b: &Board) {
    for j in 0..3 {
        for k in 0..3 {
            match b[j][k] {
                Square::X => print!("X "),
                Square::O => print!("O "),
                Square::Empty => print!(". "),
            }
        }
        println!("");
    }
                
            
}

// Checks to see who won (or if tied) based on a board.

fn check_for_win(b: &Board) -> GameStatus {

    // Number of x's and o's
    let mut xs = 0;
    let mut os = 0;

    // Go through all elements in array.  Increment
    // values for each X or O found
    
    for j in 0..3 {
        for k in 0..3 {
            match b[j][k] {
                Square::X => xs += 1,
                Square::O => os += 1,
                Square::Empty => {}
            }
        }
    }

    // Possibilities:
    //  More X's than O's -> Player 1 wins
    //  More O's than X's -> Player 2 wins
    //  Equal #s of X's and O's (rare!) -> Tie
    
    if xs > os {
        return GameStatus::Player1Win;
    } else if xs < os {
        return GameStatus::Player2Win;
    } else {
        return GameStatus::Tie;
    }
    
    
}

// Pick a random square and return it as a tuple
// Setting it to usize now because we will need it
// as usize to act as an array index anyways

fn pick_random_square() -> (usize, usize) {
    let x = rand::thread_rng()
        .gen_range(0, 3);
    let y = rand::thread_rng()
        .gen_range(0, 3);
    (x, y)
}

// For each turn, pick a random square on the board and
// modify it according to whosever turn it is

fn take_turn(p: &Player, mut b: &mut Board) {
    let (x, y) = pick_random_square();

    match p {
        &Player::X => {
            b[x][y] = Square::X;
        },
        &Player::O => {
            b[x][y] = Square::O;
        }
    }

}

fn main() {

    // The board on which tic-tac-schmoe is played
    let mut b: Board = [[Square::Empty; 3]; 3];

    // The names of the two players
    let name1;
    let name2;

    // Get and set names of the two users
    
    println!("Enter name 1 > ");
    
    match get_name() {
        Ok(n) => name1 = String::from(n.trim()),
        Err(_) => name1 = String::from("DEFAULT1")
    }

    println!("Enter name 2 > ");
    
    match get_name() {
        Ok(n) => name2 = String::from(n.trim()),
        Err(_) => name2 = String::from("DEFAULT2")
    }

    
    println!("Player 1 Name {}!", name1);
    println!("Player 2 Name {}!", name2);

    // Number of turns taken
    let mut num_turns = 0;

    // The current player.  Will flip-flop between
    // the possible values (X and O) each iteration.
    let mut current_player = Player::X;

    // Loop 100 times.  Could also have done this as:
    // for j in 0..101
    // and then we wouldn't need num_turns *shrug*
    
    while num_turns < 100 {
        num_turns += 1;

        // Take a turn and display whose turn it is
        
        match current_player {
            Player::X => println!("Player X:"),
            Player::O => println!("Player O:"),
        }
        
        take_turn(&mut current_player, &mut b);

        // Flip-flop which player is playing
        
        match current_player {
            Player::X => current_player = Player::O,
            Player::O => current_player = Player::X,
        }

        // Print out the board
        
        print_board(&b);

    }

    // Afterwards, see who won!
    // Note the beauty of matching - although ties are rare, they
    // CAN happen, and we want to ensure that we capture that
    // possibility.
    // This can't be done with a traditional switch or if statement
    
    match check_for_win(&b) {
            GameStatus::Player1Win => println!("{} won! {} lost!", name1, name2),
            GameStatus::Player2Win => println!("{} won! {} lost!", name2, name1),
            GameStatus::Tie => println!("{} and {} tie!", name1, name2),
    }
    
}
