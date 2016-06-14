extern crate num;

use std::env;
use std::process;
use std::num::ParseFloatError;

use std::mem;
use std::ptr;

use num::complex::Complex;

const ESCAPE_VAL: f64 = 2.0;
const MAX_TRIES: i32  = 50;
const X_SIZE: usize   = 40;
const Y_SIZE: usize   = 40;

fn escaped(n: &Complex<f64>) -> bool {
    false
}

fn get_initial_c(z: &Complex<f64>) -> f64 {
    let s : f64 = (z.re.powi(2) + z.im.powi(2)) as f64;
    let c : f64 = (s as f64).sqrt();
    c
}


fn iterate(z: &Complex<f64>, c: f64) -> f64 {
    (z.norm_sqr() as f64) + c
}

// A negative number indicates that it never escapes
// A positive number indicates the # of iterations before escaping

fn check_escape(z: &Complex<f64>) -> i32 {
    let mut tmp: f64 = 0.0;
    let mut c: f64 = get_initial_c(&z);
    let mut count : i32 = 0;
    while count <= MAX_TRIES && c <= 2.0 {
        c = iterate(&z, c);
        count += 1;
    }
    if count == MAX_TRIES {
        // We did not escape
        -1
    } else {
        // number of iterations it took to escape
        count
    }
}

fn mandelbrot(matrix: [[Complex<f64>; X_SIZE]; Y_SIZE]) -> [[i32; X_SIZE]; Y_SIZE] {
    let mut matrix = [[0; X_SIZE]; Y_SIZE];
    matrix
}


fn generate_matrix(x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> [[Complex<f64>; X_SIZE]; Y_SIZE] {
    let mut matrix = [[Complex {re: 0.0, im: 0.0}; X_SIZE]; Y_SIZE];
    let x_increment: f64  = ((x_max - x_min) as f64) / (X_SIZE as f64);
    let y_increment: f64  = ((y_max - y_min) as f64) / (Y_SIZE as f64);
    for j in 0..39 {
        for k in 0..39 {
            let re_val: f64 = x_min + ((j as f64) * x_increment);
            let im_val: f64 = y_min + ((k as f64) * y_increment);
            matrix[j][k] = Complex {re: re_val, im: im_val };
        }
    }
    matrix

}

fn print_matrix(matrix: [[i32; 40]; 40]) {
    for j in 0..39 {
        for k in 0..39 {
            if matrix[j][k] > 0 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    
}

fn print_usage_and_exit() -> ! {
    println!("Usage: mandelbrot <x_min> <x_max> <y_min> <y_max>");
    println!("x_min must be less than x_max");
    println!("y_min must be less than y_max");
    process::exit(1);
}


// TODO: doesn't work with negatives!
fn read_args(args: Vec<String>) -> Result<(f64, f64, f64, f64), ParseFloatError> {

    let x_min = try!(args[1].parse::<f64>());
    let x_max = try!(args[2].parse::<f64>());
    let y_min = try!(args[3].parse::<f64>());
    let y_max = try!(args[4].parse::<f64>());

    if x_min >= x_max || y_min >= y_max {
        print_usage_and_exit();
    } 
    
    Ok((x_min, x_max, y_min, y_max))
}

fn main() {
    
    println!("Starting..");
    
    let args: Vec<_> = env::args().collect();
    if args.len() != 5 {
        print_usage_and_exit();
    }
    let (x_min, x_max, y_min, y_max) =
        read_args(args).expect("Could not read args!");

    println!("Drawing from {}, {} to {}, {}",
             x_min,
             y_min,
             x_max,
             y_max);

}
