fn main() {
    type Kilometers = i32;

    let x: i32 = 23;
    let y: Kilometers = 50;

    println!("x + y = {}", x + y);

    generic("test");

    let z = test_never();
    println!("This can't be printed");
}

fn test_never() -> ! {
    panic!()
}

fn generic<T: ?Sized>(t: &T) {
    println!("This function can take types without a known size");
}