use crate::garden::vegetables::Asparagus;

mod garden;

fn main() {
    let plant: Asparagus = Asparagus{
        length: 4,
        ripeness: 100,
    };
    println!("plant is {:#?}", plant);
}
