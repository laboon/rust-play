extern crate rand;

use std::env;
use std::process;
use std::io;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use rand::Rng;


fn count_neighbors(x: i32, y: i32) -> i32 {
    1
}

fn print_usage_and_exit() -> ! {
    println!("Usage: life <x_size> <y_size> <percent_alive>");
    println!("All must be positive integers.");
    println!("percent_alive must be between 0 and 100 (inclusive)");
    process::exit(1);
}

fn get_random_living(pct: i32) -> bool {
    let num : i32 = rand::thread_rng().gen_range(1, 100);
    num <= pct
}

fn print_world(w: &mut [[i32; 20]; 20]) {
    for j in 0..w.len() {
        let inner = &w[j];
        for k in 0..inner.len() {
            if inner[k] == 0 {
                print!(".");
            } else {
                print!("X");
            }
        }
        println!("");
    }
    println!("");
    
}

fn save_world(w: &mut [[i32; 20]; 20]) {

    let path = Path::new("saved_world.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           Error::description(&why)),
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


fn iterate_world(w: &mut [[i32; 20]; 20]) {
    
}

fn generate_world(w: &mut [[i32; 20]; 20], pct: i32) {

    for j in 0..w.len() {
        let mut inner = &mut w[j];
        for k in 0..inner.len() {
            let living : bool = get_random_living(pct);
            if living {
                inner[k] = 1;
            } else {
                // leave at 0
            }
        }
    }
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

    println!("IGNORING ARGS TO MAKE A FIXED-SIZE ARRAY!");
    
    println!("Creating a {} x {} array, {}% alive", 20, 20, pct);

    
    
    // There has to be a better way to do this
    // from http://stackoverflow.com/questions/13212212/creating-two-dimensional-arrays-in-rust
    // let x_usize = x as usize;
    // let y_usize = y as usize;
    // let mut grid_raw = vec![0; x_usize * y_usize];
    // let mut grid_base: Vec<i32> = grid_raw.as_mut_slice().chunks_mut(x_usize).collect();
    // let mut grid: &mut [&mut [_]] = grid_base.as_mut_slice();
    
    let mut choice = String::new();

    let mut cont = true;

    let mut world = &mut [[0; 20]; 20];
    generate_world(world, pct);
    print_world(world);
    
    while cont {

        print!("[N]ext, [S]ave, [Q]uit > ");
        io::stdout().flush().ok().expect("Could not flush stdout");
        
        io::stdin().read_line(&mut choice);
        
        match choice.trim() {
            "Q" | "q" => cont = false,
            "N" | "n" => cont = true,
            "S" | "s" => {
                save_world(world);
                cont = true
            },
            _         => println!("Please choose a valid option!"),
        }

        if cont {
            iterate_world(world);
            print_world(world);
            choice.clear();

        }
        
    }
}
