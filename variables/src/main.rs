fn main() {
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let tup = (500, 6.4, 1);

    let (_x, _y, _z) = tup;

    println!("The value of y is: {_y}");
    
    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;
}
