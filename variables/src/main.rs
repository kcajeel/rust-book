fn main() {
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let x = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
