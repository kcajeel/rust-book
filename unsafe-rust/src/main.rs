use core::slice;

fn main() {
    // raw pointers
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0xFFFFFFusize;
    let r3 = address as *const i32;

    // calling a C function
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    // dereferencing raw pointers and calling unsafe functions and external functions are unsafe
    unsafe {
        *r2 = 10;
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        // print!("r3 is: {}", *r3); // this line will panic
        dangerous();
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // create a function using unsafe code
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    // let (a, b) = r.split_at_mut(3);
    let (a, b) = split_at_mut(&mut v, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // modifying static variables is unsafe
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

static mut COUNTER: u32 = 0;

unsafe fn dangerous() {}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// unsafe traits
unsafe trait Foo {
    // stuff here
}

unsafe impl Foo for i32 {
    // more stuff here
}