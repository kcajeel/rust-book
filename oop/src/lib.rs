pub trait Draw {
    fn draw(&self);
}

// the component vec uses a trait object to contain any type that implements Draw
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to draw the button
        println!("Button's draw method was called");
    }
}
