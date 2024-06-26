mod front_of_house;

mod back_of_house;

pub struct Item {
    name: String,
    price: i32,
}

pub enum MenuItem {
    Appetizer(Item),
    Entree(Item),
    Drink(Item),
    Desert(Item),
}

pub fn eat_at_restaurant() {
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();

    let menu = front_of_house::hosting::Menu::new("calamari".to_owned(), "pizza".to_owned(), "cake".to_owned());
    let entrees = front_of_house::hosting::Menu::get_entrees(&menu);
    let appetizers = menu.appetizers;
}