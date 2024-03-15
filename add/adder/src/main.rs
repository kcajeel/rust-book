use add_one;
use add_two;
use rand;

fn main() {
    let num = 10;
    println!("Hello, world! 
    {num} plus one is {}! 
    {num} plut two is {}!", 
    add_one::add_one(num), 
    add_two::add_two(num));
}