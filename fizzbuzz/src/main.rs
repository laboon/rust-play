fn fizzy(n: i32) -> bool {
    n % 3 == 0
}

fn buzzy(n: i32) -> bool {
    n % 5 == 0
}

fn main() {
    for j in 1..100 {
        if fizzy(j) && buzzy(j) {
            println!("FizzBuzz");
        } else if fizzy(j) {
            println!("Fizz");
        } else if buzzy(j) {
            println!("Buzz");
        } else {
            println!("{}", j);
        }
    }
}
