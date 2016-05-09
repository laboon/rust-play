use std::env;
use std::process;

// fn count_neighbors(x: i32, y: i32) -> i32 {
//     1
// }

fn print_usage_and_exit() {
    println!("Usage: life <x_size> <y_size> <percent_alive>");
    println!("All must be positive integers.");
    println!("percent_alive must be between 0 and 100 (inclusive)");
    process::exit(1);
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
    
    
    (*x, *y, *pct)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        print_usage_and_exit();
    }
    let (x, y, pct) = read_args(args);

    println!("Creating a {} x {} array, {}% alive", x, y, pct);

    // let world = [[bool; x]; y];
}
