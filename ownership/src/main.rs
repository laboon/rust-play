fn take(v: Vec<i32>) {
    println!("took the vector!");
}

fn change_truth(x: bool) -> bool {
    !x
}

fn fluffy(v1: &Vec<i32>, v2: &Vec<i32>) -> bool {
    if (v1[0] == 1) {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let v = vec![1,2,3];
    // let v2 = v;
    // take(v);
    println!("v[0] is {}", v[0]);

    let b: i32 = 1;
    let b2: i32 = b;

    println!("b is {}", b);

    let a = true;
    let not_a = change_truth(a);
    println!("a is {}, not_a is {}", a, not_a);

    let c1 = vec![1,2,3];
    let c2 = vec![1,2,3];
    let fluffy = fluffy(&c1, &c2);
    println!("fluffy is {}", fluffy);

    println!("c2[1] is {}", c2[1]);

    let mut m = 17;
    m = 14;
    {
        let m2 = &mut m;
        *m2 += 1;
        
    }
    println!("mut m is {}", m);
    
}
