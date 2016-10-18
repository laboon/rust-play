extern crate quickcheck;

use quickcheck::quickcheck;
use quickcheck::TestResult;

// Integer absolute value function

fn abs(x: i32) -> i32 {
    if x >= 0 {
        x
    } else {
        x * -1
    }
}

// Floating-point absolute value function

fn fabs(x: f32) -> f32 {
    if x >= 0.0 {
        x
    } else {
        x * -1.0
    }
}

// Square root function

fn sqrt(x: f32) -> f32 {
    let threshold = 0.0000001;
    let val = fabs(x);
    let mut low: f32 = 0.0;
    let mut high: f32 = val;
    let mut mid: f32 = val;
    let mut old: f32 = -1.0;
    let mut midsqr: f32;
    
    while fabs(old - mid) > threshold {
        old = mid;
        mid = (high + low) / 2.0;
        midsqr = mid * mid;
        // print!("{} {} {} {} {} ",
        //          val, low, high, mid, midsqr);
        if midsqr > val {
            high = mid;
            // println!("- HIGH");
        } else {
            low = mid;
            //println!("- LOW");
        }
    }
    mid
}

// Selection sort function - given an array, will
// sort in ascending order

fn selection_sort(mut a: &mut [i32]) {
    
    // if 0 or 1 elements, can just return
    if a.len() < 2 {
        return
    }

    let mut min_index: usize;
    let mut min_val: i32;

    for j in 0..a.len() {
        // println!("Checking position {}", j);
        min_index = j;
        min_val = a[j];
        for k in (j+1)..a.len() {
            // println!("\t{} - Comparing {} & {}", k, min_val, a[k]);
            if a[k] < min_val {
                // println!("\tSetting min_val to {}", a[k]);
                min_val = a[k];
                min_index = k;
            }
        }
        // println!("{} < {} ?", a[min_index], min_val);
        // println!("Swapping locs {} & {}", j, min_index);
        swap(&mut a, j, min_index)

    }
}



// Swap function - given an array a and two locations (p1 & p2),
// will swap the values in a[p1] and a[p2]

fn swap(a: &mut [i32], p1: usize, p2: usize) {
    if p1 != p2 {
        let tmp: i32 = a[p1];
        a[p1] = a[p2];
        a[p2] = tmp;
    }
}

fn safe_divide(n: f32, d: f32) -> Option<f32> {
    if d == 0.0 {
        None
    } else {
        let val = n / d;
        Some(val)
    }
}

fn main() {
    let xs: [f32; 10] = [1.0, 2.0, 3.0, 4.0, 5.0,
    6.0, 7.0, 8.0, 9.0, 10.0];
    let mut res: f32;
    for x in &xs {
        res = sqrt(*x);
        println!("sqrt({}) = {}", x, res);
    }

    let mut sort_xs: [i32; 10] = [9, 6, -9, 2, 1, 3, 18, 12, 7, 66];
    selection_sort(&mut sort_xs);
    println!("{:?}", sort_xs);

}



// A classic unit test, for comparison

#[test]
fn test_abs() {
    assert_eq!(1, abs(1));
}

// A property-based test.  Note that there are no
// explicit assertions - they are "built-in" to the
// quickcheck function.  However, they can be
// annotated as #[test] just like a normal unit test.

// Absolute values should always be positive

#[test]
fn test_abs_prop() {
    // Define a property or properties here.
    // For Rust's version of QuickCheck, I have found
    // that it's better to define one property per test.
    // This makes reading the output much easier.

    // These properties can also be defined elsewhere
    // if you would like to re-use them.  However,
    // I find that keeping them in the test block
    // is easiest for understanding.
    
    fn prop_no_neg(x: i32) -> bool {
        abs(x) >= 0
    }

    // Now run with many random i32 values and check
    // that the property holds true (i.e., the function
    // prop_no_neg returns true) for all of them.
    // If not, the test will fail.

    // Note that you need to have the "as" section
    quickcheck(prop_no_neg as fn(i32) -> bool);
}

// Absolute values should always be positive

#[test]
fn test_fabs_no_neg() {

    fn prop_fabs_no_neg(x: f32) -> bool {
        fabs(x) >= 0.0
    }

    quickcheck(prop_fabs_no_neg as fn(f32) -> bool);
}


// Absolute values should always return same value

#[test]
fn test_fabs_always_same() {

    fn prop_fabs_always_same(x: f32) -> bool {
        fabs(x) == fabs(x)
    }
    quickcheck(prop_fabs_always_same as fn(f32) -> bool);
}

// Absolute values should be idempotent

#[test]
fn test_fabs_idempotent() {
    fn prop_fabs_idempotent(x: f32) -> bool {
        fabs(x) == fabs(fabs(x))
    }
    quickcheck(prop_fabs_idempotent as fn(f32) -> bool);
}    


// Square roots should always return a positive value

#[test]
fn test_sqrt_prop_no_neg() {

    fn prop_sqrt_no_neg(x: f32) -> bool {
        sqrt(x) >= 0.0
    }

    quickcheck(prop_sqrt_no_neg as fn(f32) -> bool);   
}

// Squaring a square root should get you the
// original number.  Note that since these are
// floating-point numbers, we will need to take
// FP errors into consideration.  Arbitrarily,
// we will choose an epsilon value of 0.001.


#[test]
fn test_sqrt_prop_squaring() {

    fn prop_sqrt_squaring(x: f32) -> bool {
        let rt = sqrt(x);
        (rt * rt) - fabs(x) < 0.001
    }
    quickcheck(prop_sqrt_squaring as fn(f32) -> bool);
}

// The square root should always be less than the
// absolute value of the original value

#[test]
fn test_sqrt_prop_lte_orig() {

    fn prop_sqrt_lte_orig(x: f32) -> bool {
        sqrt(x) <= fabs(x)
    }
    
    quickcheck(prop_sqrt_lte_orig as fn(f32) -> bool);
}

// safe_divide-ing with a 0 as the denominator should
// always return None.  Note that we're generating only the
// x (numerator) and keeping the denominator constant.

#[test]
fn test_safe_divide_zero_denom() {

    fn prop_sd_div_0_none(x: f32) -> bool {
        let r = safe_divide(x, 0.0);
        match r {
            Some(_) => false,
            None => true
        }
    }
    quickcheck(prop_sd_div_0_none as fn(f32) -> bool);
}
    
// safe-dividing with any other value as the denominator
// should return Some value

// This is more complicated!  We need to throw away any
// randomly generated pair where y is equal to 0.0.
// First, note that we are generating TWO different
// arguments to pass in (this necessitates changing the "as"
// section to match the function signature).

// Secondly, we need to say that some tests are invalid
// (any where y == 0).  We can return a TestResult instead of
// a regular old boolean value, but this will complicate things
// a bit.  

#[test]
fn test_safe_divide_nonzero_denom() {

    // Note that we are returning a TestResult here
    fn prop_sd_div_nonzero_some(x: f32, y: f32) -> TestResult {
        if y == 0.0 {
            TestResult::discard()
        } else {            
            let r = safe_divide(x, y);
            match r {
                Some(_) => TestResult::from_bool(true),
                None => TestResult::from_bool(false)
            }
        }
    }
    
    // we are returning a TestResult here as well
    quickcheck(prop_sd_div_nonzero_some as fn(f32, f32) -> TestResult);
}

// Shrinking

