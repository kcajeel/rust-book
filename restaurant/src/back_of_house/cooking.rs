fn receive_order() {}

    fn cook_food() {}

    fn plate_food() {}

    fn fix_incorrect_order() {
        cook_food();
        crate::front_of_house::serving::serve_order();
    }