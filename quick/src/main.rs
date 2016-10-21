extern crate quickcheck;

use quickcheck::quickcheck;
use quickcheck::TestResult;

// Integer absolute value function

fn abs(x: i32) -> i32 {
    if x.is_negative() { -x } else { x }
}

// Floating-point absolute value function

fn fabs(x: f32) -> f32 {
    if x.is_sign_negative() { -x } else { x }
}

// Square root function

fn sqrt(x: f32) -> f32 {
    let threshold = 0.0000001;
    let val = fabs(x);
    let mut low = 0.0;
    let mut high = val;
    let mut mid = val;
    let mut old = -1.0;
    let mut midsqr;

    while fabs(old - mid) > threshold {
        old = mid;
        mid = (high + low) / 2.0;
        midsqr = mid.powi(2);
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

// Selection sort function - given a Vector of i32's, will
// sort in ascending order if asc = true, else descending

fn selection_sort_vec(v: &mut [i32], asc: bool) {

    // if 0 or 1 elements, can just return
    if v.len() < 2 {
        return
    }

    let mut min_index;
    let mut min_val;

    for j in 0..v.len() {
        // println!("Checking position {}", j);
        min_index = j;
        min_val = v[j];
        for (k, &i) in v.iter().enumerate().skip((j + 1)) {
            // println!("\t{} - Comparing {} & {}", k, min_val, a[k]);
            if asc {
                if i < min_val {
                    // println!("\tSetting min_val to {}", a[k]);
                    min_val = i;
                    min_index = k;
                }
            } else if v[k] > min_val {
                // println!("\tSetting min_val to {}", a[k]);
                min_val = i;
                min_index = k;
            }
        }
        // println!("{} < {} ?", v[min_index], min_val);
        // println!("Swapping locs {} & {}", j, min_index);
        v.swap(j, min_index)

    }
}

// Divides two f32's, and wraps the return value in an Option

fn safe_divide(n: f32, d: f32) -> Option<f32> {
    if d == 0.0 {
        None
    } else {
        Some(n / d)
    }
}

//////////////////////////////////////////////////////////////////
// Boring main function which runs through a few functions
//////////////////////////////////////////////////////////////////

fn main() {
    let d = safe_divide(7.0, 22.0);
    match d {
        Some(n) => println!("Finite number {}", n),
        None    => println!("INFINITY"),
    }

    let q = abs(-7);
    println!("q is {}", q);

    for x in 1..11 {
        let res = sqrt(x as f32);
        println!("sqrt({}) = {}", x, res);
    }

    let mut sort_vec = vec![13, 8, 9, 12, 90, 87, 63, 22];
    selection_sort_vec(&mut sort_vec, true);
    println!("{:?}", sort_vec);

    selection_sort_vec(&mut sort_vec, false);
    println!("{:?}", sort_vec);
}



// A classic unit test, for comparison.
// Just checks that the absolute value of -1 is 1.
// Note that we are only checking one specific value
// and one specific output.

#[test]
fn test_abs() {
    assert_eq!(1, abs(-1));
}

////////////////////////////////////////////////////////////////
// abs(i32) tests
////////////////////////////////////////////////////////////////


// A property-based test.  Note that there are no
// explicit assertions - they are "built-in" to the
// quickcheck function.  However, they can be
// annotated as #[test] just like a normal unit test.

// Absolute values should never be negative

#[test]
fn test_abs_never_negative() {
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

////////////////////////////////////////////////////////////////
// fabs(i32) tests
////////////////////////////////////////////////////////////////


// Absolute values should always be positive

#[test]
fn test_fabs_no_neg() {

    fn prop_fabs_no_neg(x: f32) -> bool {
        !fabs(x).is_sign_negative()
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

////////////////////////////////////////////////////////////////
// sqrt tests
////////////////////////////////////////////////////////////////


// Square roots should always return a positive value

#[test]
fn test_sqrt_prop_no_neg() {

    fn prop_sqrt_no_neg(x: f32) -> bool {
        !sqrt(x).is_sign_negative()
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
        rt.powi(2) - fabs(x) < 0.001
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

////////////////////////////////////////////////////////////////
// safe_divide tests
////////////////////////////////////////////////////////////////


// safe_divide-ing with 0.0 as the denominator should
// always return None.  Note that we're generating only the
// x (numerator) and keeping the denominator constant.

#[test]
fn test_safe_divide_zero_denom() {

    fn prop_sd_div_0_none(x: f32) -> bool {
        safe_divide(x, 0.0).is_none()
    }
    quickcheck(prop_sd_div_0_none as fn(f32) -> bool);
}

// safe-dividing with any nonzero value as the denominator
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

// Finally, note that values are generated and then discarded
// later.  If too many values are generated which are discarded,
// quickcheck will eventually give up (specifically, if it can't
// find 100 examples after 10,000 tries, it will stop trying).

#[test]
fn test_safe_divide_nonzero_denom() {

    // Note that we are returning a TestResult here
    fn prop_sd_div_nonzero_some(x: f32, y: f32) -> TestResult {
        if y == 0.0 {
            TestResult::discard()
        } else {
            let r = safe_divide(x, y).is_some();
            TestResult::from_bool(r)
        }
    }

    // we are returning a TestResult here as well
    quickcheck(prop_sd_div_nonzero_some as fn(f32, f32) -> TestResult);
}

////////////////////////////////////////////////////////////////
// selection_sort_vec tests
////////////////////////////////////////////////////////////////

// Check that the last value in an ascending sort is not smaller than
// any other value in the vector

#[test]
fn test_selection_sort_asc_last_element() {
    // These muts in the test fn signatures *are* needed and are fine since you're taking ownership of v
    fn prop_last_element_not_smaller(mut v: Vec<i32>) -> TestResult {
        if v.len() < 2 {
            TestResult::discard()
        } else {
            selection_sort_vec(&mut v, true);

            let (head, tail) = v.split_at(v.len() - 1);
            let last_elem = tail[0];

            let all_smaller = head.iter().all(|&x| x <= last_elem);
            TestResult::from_bool(all_smaller)
        }
    }
    quickcheck(prop_last_element_not_smaller as fn(Vec<i32>) -> TestResult);
}

// Sorting an array twice should return the same result as doing it
// once [idempotency]

#[test]
fn test_selection_sort_idempotent() {

    fn prop_idempotent(mut once: Vec<i32>) -> bool {
        selection_sort_vec(&mut once, true);
        let mut twice = once.clone();
        selection_sort_vec(&mut twice, true);

        once == twice
    }

    quickcheck(prop_idempotent as fn(Vec<i32>) -> bool);
}
