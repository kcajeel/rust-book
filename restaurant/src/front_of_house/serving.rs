use crate::Item;
fn take_order() -> Vec<Item> {
    let order : Vec<Item> = Vec::new();
    order
}

pub fn serve_order() {}

fn calculate_price(order: impl Iterator<Item = Item>) -> i32 {
    order.map(|i| i.price).sum()    
}

fn take_payment() {}