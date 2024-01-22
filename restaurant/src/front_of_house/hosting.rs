pub struct Menu {
pub appetizers: Vec<String>,
pub entrees: Vec<String>,
pub desserts: Vec<String>,
}

impl Menu {
    pub fn new(this_appetizer: String, this_entree: String, this_dessert: String) -> Self{
        Self {
            appetizers: vec![this_appetizer],
            entrees: vec![this_entree],
            desserts: vec![this_dessert],
        }
    }

    pub fn get_entrees(menu: &Menu) -> Vec<String> {
        menu.entrees.clone()
    }
}

pub fn add_to_waitlist() {}

fn seat_at_table() {}
