extern crate num;

use std::env;
use std::process;
use std::num::ParseIntError;

use std::mem;
use std::ptr;

use num::complex::Complex;

const ESCAPE_VAL: i32 = 2;
const MAX_TRIES: i32  = 50;

fn escaped(n: &Complex<i32>) -> bool {
    false
}

fn get_initial_c(z: &Complex<i32>) -> f64 {
    let s : f64 = (z.re.pow(2) + z.im.pow(2)) as f64;
    let c : f64 = (s as f64).sqrt();
    c
}


fn iterate(z: &Complex<i32>, c: f64) -> f64 {
    (z.norm_sqr() as f64) + c
}

// A negative number indicates that it never escapes
// A positive number indicates the # of iterations before escaping

fn check_escape(z: &Complex<i32>) -> i32 {
    let mut tmp: i32 = 0;
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

fn print_usage_and_exit() -> ! {
    println!("Usage: mandelbrot <x_min> <x_max> <y_max> <y_min>");
    println!("x_min must be less than x_max");
    println!("y_min must be less than y_max");
    process::exit(1);
}

fn generate_matrix(x_min: i32, x_max: i32, y_min: i32, y_max: i32) -> [[Complex<i32>; 40]; 40] {
    // TODO: create a 2-d array of complex numbers
    let mut matrix = [[Complex {re: 0, im: 0}; 40]; 40];
    for j in 0..39 {
        for k in 0..39 {
            matrix[j][k] = Complex::new(j as i32, k as i32);
        }
    }
    matrix

}

fn read_args(args: Vec<String>) -> Result<(i32, i32, i32, i32), ParseIntError> {

    let x_min = try!(args[1].parse());
    let x_max = try!(args[2].parse());
    let y_min = try!(args[3].parse());
    let y_max = try!(args[4].parse());

    if x_min >= x_max || y_min >= y_max {
        print_usage_and_exit();
    } 
    
    Ok((x_min, x_max, y_min, y_max))
}

// fn draw() {
// }


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
