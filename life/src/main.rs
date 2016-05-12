extern crate rand;

use std::env;
use std::process;
use std::io;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use std::num::ParseIntError;

use rand::Rng;

// A world is a 20x20 array

type World = [[i32; 20]; 20];

// Given some world and a location in it (x, y coords), return the
// number of living neighbors

fn count_neighbors(w: &World, x: i32, y: i32) -> i32 {

    let size : i32 = w.len() as i32;
    let mut left_x = (x - 1) % size;
    let mut right_x = (x + 1) % size;
    let mut up_y = (y - 1) % size;
    let mut down_y = (y + 1) % size;

    if left_x == -1 { left_x = size - 1; }
    if right_x == -1 { right_x = size - 1; }
    if up_y == -1 { up_y = size - 1; }
    if down_y == -1 { down_y = size - 1; }
    
    let mut num_neighbors = 0;

    if w[left_x as usize][up_y as usize] != 0    { num_neighbors += 1; }
    if w[left_x as usize][down_y as usize] != 0  { num_neighbors += 1; }
    if w[left_x as usize][y as usize] != 0       { num_neighbors += 1; }
    if w[right_x as usize][up_y as usize] != 0   { num_neighbors += 1; }
    if w[right_x as usize][down_y as usize] != 0 { num_neighbors += 1; }
    if w[right_x as usize][y as usize] != 0      { num_neighbors += 1; }
    if w[x as usize][up_y as usize] != 0         { num_neighbors += 1; }
    if w[x as usize][down_y as usize] != 0       { num_neighbors += 1; }

    num_neighbors
}


fn print_usage_and_exit() -> ! {
    println!("Usage: life <x_size> <y_size> <percent_alive>");
    println!("All must be positive integers.");
    println!("percent_alive must be between 0 and 100 (inclusive)");
    process::exit(1);
}

fn get_random_living(pct: i32) -> bool {
    let num = rand::thread_rng().gen_range(0, 100);
    num <= pct
}

fn print_world(w: &World) {
    for inner in w {
        for &elem in inner {
            print!("{}", if elem == 0 { "." } else { "X" });
        }
        println!("");
    }
    println!("");
    
}

fn save_world(w: &World) {

    let path = Path::new("saved_world.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };


    for j in 0..w.len() {
        let inner = &w[j];
        for k in 0..inner.len() {
            let to_write = match inner[k] {
                0 => ".",
                _ => "X",
            };
            match file.write_all(to_write.as_bytes()) {
                Err(why) => {
                    panic!("couldn't write to {}: {}", display,
                           Error::description(&why))
                },
                Ok(_) => (),
            };

        }
        match file.write_all("\n".as_bytes()) {
            Err(why) => {
                panic!("couldn't write to {}: {}", display,
                       Error::description(&why))
            },
            Ok(_) => (),
        };

    }
    println!("Successfully saved world to {}", display);
    
}

// Check if cell should be alive or dead next iteration

fn alive(w: &World, x: i32, y: i32) -> i32 {
    // If already alive, and has 2 or 3 neighbors, alive, else dead
    // If dead, and has exactly 3 neighbors, alive, else dead
    let num_neighbors = count_neighbors(w, x, y);

    if w[x as usize][y as usize] == 0 {
        // currently dead
        return match num_neighbors {
            3 => 1,
            _ => 0,
        };
    } else {
        // currently alive
        return match num_neighbors {
            2 | 3 => 1,
            _     => 0,
        };
    }
}

// Run one iteration of the game.
// Go through each element

fn iterate_world(w: World) -> World {

    let mut new_world = [[0; 20]; 20];
    for j in 0..w.len() {
        for k in 0..w[0].len() {
            new_world[j][k] = alive(&w, j as i32, k as i32);
        }
    }
    new_world
    
}

fn generate_world(w: &mut World, pct: i32) {

    for j in 0..w.len() {
        let mut inner = &mut w[j];
        for k in 0..inner.len() {
            if get_random_living(pct) {
                inner[k] = 1;
            } else {
                // leave at 0
            }
        }
    }
}

fn read_args(args: Vec<String>) -> Result<(i32, i32, i32), ParseIntError> {

    let x = try!(args[1].parse());
    let y = try!(args[2].parse());
    let pct = try!(args[3].parse());

    Ok((x, y, pct))
}

fn main() {

    // Get arguments from command line.
    // Currently ignoring the first two because I don't want to deal with
    // crazy dynamic arrays in Rust right now.
    
    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        print_usage_and_exit();
    }
    let (_x, _y, pct) = read_args(args).expect("Could not read args!");

    println!("Creating a {} x {} array, {}% alive", 20, 20, pct);

    
    
    // There has to be a better way to do dynamic arrays!
    // from http://stackoverflow.com/questions/13212212/creating-two-dimensional-arrays-in-rust
    // let x_usize = x as usize;
    // let y_usize = y as usize;
    // let mut grid_raw = vec![0; x_usize * y_usize];
    // let mut grid_base: Vec<i32> = grid_raw.as_mut_slice().chunks_mut(x_usize).collect();
    // let mut grid: &mut [&mut [_]] = grid_base.as_mut_slice();


    // The choice coming in from the user each turn
    let mut choice = String::new();

    // Whether or not to continue running
    let mut cont = true;

    // The (20 x 20) fixed array which makes up the world
    let mut world = [[0; 20]; 20];

    // Generate the world (random) and print it out to start
    generate_world(&mut world, pct);
    print_world(&world);
    
    while cont {

        // Get user's choice of what to do.  This should definitely go 
        // into a function.
        print!("[N]ext, [S]ave, [Q]uit > ");
        io::stdout().flush().expect("Could not flush stdout");
        io::stdin().read_line(&mut choice).expect("Could not read line");
        
        match choice.trim() {
            "Q" | "q" => cont = false, // Quit
            "N" | "n" => cont = true,  // Next iteration
            "S" | "s" => {             // Save to file
                save_world(&world);
                // don't iterate!
                cont = true
            },
            _         => println!("Please choose a valid option!"), // Try again!
        }

        // If user is continuing, iterate the world one iteration and print it out
        // Then clear the choice string so it can re-used.  Otherwise user input
        // is just appended.  If we are not continuing, the program is exiting, so
        // no need to clear it out.
        
        if cont {
            world = iterate_world(world);
            print_world(&world);
            choice.clear();

        }
        
    }
}



#[test]
fn count_neighbors_dead_world() {
    let test_world = [[0; 20]; 20]; 
    let n = count_neighbors(&test_world, 1, 1);
    
    assert_eq!(0, n);
}

#[test]
fn count_neighbors_one_alive() {
    let mut test_world = [[0; 20]; 20];
    test_world[0][0] = 1;
    let n = count_neighbors(&test_world, 1, 1);
    
    assert_eq!(1, n);
}

#[test]
fn count_neighbors_all_alive() {
    let test_world = [[1; 20]; 20];
    let n = count_neighbors(&test_world, 1, 1);
    
    assert_eq!(8, n);
}

// #[test]
// fn alive() {
//     let test_world = [[1; 20]; 20];
//     let n = count_neighbors(&test_world, 1, 1);
    
//     assert_eq!(8, n);
// }
