extern crate rand;

use std::env;
use std::process;
use std::io;
use std::io::Write;

use rand::Rng;


// fn count_neighbors(x: i32, y: i32) -> i32 {
//     1
// }

fn print_usage_and_exit() {
    println!("Usage: life <x_size> <y_size> <percent_alive>");
    println!("All must be positive integers.");
    println!("percent_alive must be between 0 and 100 (inclusive)");
    process::exit(1);
}

fn generate_seed() -> i32 {
    let seed : i32 = rand::thread_rng().gen_range(1, 101);
    println!("Seed is {}", seed);
    seed
}

fn create_world() {
    let seed = generate_seed();
}

fn read_args(args: Vec<String>) -> (i32, i32, i32) {

    let x = &args[1].parse::<i32>().unwrap();
    let y = &args[2].parse::<i32>().unwrap();
    let pct = &args[3].parse::<i32>().unwrap();

    
    // TODO: Figure error-handling out
    // let x_opt = &args[1].parse::<i32>();
    // let x = match x_opt {
    //     Err(e)   => -1,
    //     Ok(n)    => n,
    // };

    // TODO: Why do I need pointers here?  Because of &args???
    
    (*x, *y, *pct)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        print_usage_and_exit();
    }
    let (x, y, pct) = read_args(args);

    println!("Creating a {} x {} array, {}% alive", x, y, pct);

    // There has to be a better way to do this
    // from http://stackoverflow.com/questions/13212212/creating-two-dimensional-arrays-in-rust
    let x_usize = x as usize;
    let y_usize = y as usize;
    let mut grid_raw = vec![0; x_usize * y_usize];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(x_usize).collect();
    let mut grid: &mut [&mut [_]] = grid_base.as_mut_slice();
    
    // How to have this read in and mutated? =(    
    let mut choice = String::new();

    let mut cont = true;
    
    while cont {

        print!("[N]ext, [Q]uit > ");
        io::stdout().flush().ok().expect("Could not flush stdout");
        
        io::stdin().read_line(&mut choice);

        println!("You typed: {}", choice.trim());

        match choice.trim() {
            "Q" => cont = false,
            "N" => cont = true,
            _ => println!("Please choose a valid option!"),
        }

        choice.clear();

        
    }
}
