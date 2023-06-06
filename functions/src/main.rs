fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');

    let x = five();
    println!("the value of x is: {x}");

    let five_factorial = factorial(x);

    println!("the factorial of x is: {five_factorial}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

//for fun
fn factorial(n: i32) -> i32 {
    match n {
        0 | 1 => 1,
        _ => factorial(n - 1) * n,
    }
}
