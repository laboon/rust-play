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


struct World {
    cells: [[i32; 20]; 20],
}

impl World {
    fn count_neighbors(&self, x: i32, y: i32) -> i32 {
        let size : i32 = self.cells.len() as i32;
        let mut left_x = (x - 1) % size;
        let mut right_x = (x + 1) % size;
        let mut up_y = (y - 1) % size;
        let mut down_y = (y + 1) % size;

        if left_x == -1 { left_x = size - 1; }
        if right_x == -1 { right_x = size - 1; }
        if up_y == -1 { up_y = size - 1; }
        if down_y == -1 { down_y = size - 1; }
        
        let mut num_neighbors = 0;

        if self.cells[left_x as usize][up_y as usize] != 0    { num_neighbors += 1; }
        if self.cells[left_x as usize][down_y as usize] != 0  { num_neighbors += 1; }
        if self.cells[left_x as usize][y as usize] != 0       { num_neighbors += 1; }
        if self.cells[right_x as usize][up_y as usize] != 0   { num_neighbors += 1; }
        if self.cells[right_x as usize][down_y as usize] != 0 { num_neighbors += 1; }
        if self.cells[right_x as usize][y as usize] != 0      { num_neighbors += 1; }
        if self.cells[x as usize][up_y as usize] != 0         { num_neighbors += 1; }
        if self.cells[x as usize][down_y as usize] != 0       { num_neighbors += 1; }

        num_neighbors

    }
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


fn main() {
    let w = World { cells: [[0; 20]; 20] };
}

#[test]
#[test]
fn count_neighbors_dead_world() {
    let test_world = World { cells: [[0; 20]; 20] };
    let n = test_world.count_neighbors(1, 1);
    
    assert_eq!(0, n);
}

#[test]
fn count_neighbors_one_alive() {
    let mut test_world = World { cells: [[0; 20]; 20] };
    test_world.cells[0][0] = 1;
    let n = test_world.count_neighbors(1, 1);

    assert_eq!(1, n);
}

#[test]
fn count_neighbors_all_alive() {
    let test_world = World { cells: [[1; 20]; 20] };
    let n = test_world.count_neighbors(1, 1);
    
    assert_eq!(8, n);
}


