    #[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//if you're using linux i highly recommend using the mold linker, it improves compile times at least 10x
// - Drew

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
       self.area() > rect.area()
    }

    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

fn main() {

    // let a = "dog".to_string();
    // {
    //     let b = &a;
    // }
    // print!("A is: {}", a);

    print!("build_user: {:#?}\n", build_user("user1", "user1@example.com"));
    print!("build_user_shorthand: {:#?}\n", build_user_shorthand(String::from("user2"), String::from("user2@example.com")));

    //now with toString
    println!("build_user: {}", to_string(build_user("user3", "user3@example.com")));
    println!("build_user_shorthand: {}", to_string(build_user_shorthand(String::from("user4"), String::from("user4@example.com"))));

    //Example program begin here
    let width = 10;
    let height = 90;
    let rect1   = (20,20);
    let rect2 = Rectangle{
        width: 30,
        height: 50,
    };
    let rect3 = Rectangle{
        width: 10,
        height: 40,
    };
    let rect4 = Rectangle{
        width: 60,
        height: 45,
    };
    let square = Rectangle::square(5);
    for i in 1..4{
       println!( "The area of rectangle {} is {} square pixels.",i, 
       match i {
        1 => area(width,height),
        2 => area2(rect1),
        3 => area3(&rect2),
        4 => rect2.area(),
        _ => 0,
       }
        )
    }
    println!("rect2 is {:#?}", rect2);

    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect2 hold rect4? {}", rect2.can_hold(&rect4));
    println!("square is {:#?}", square);
        
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn build_user(username: &str, email: &str) -> User{
    User{
        username: username.to_string(),
        email: email.to_string(),
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_shorthand(username: String, email: String) -> User{
    let active = true;
    User{
        username,
        email,
        active,
        sign_in_count: 1,
    }
}

fn to_string(user: User) -> String{
    format!("{}, {}, {}, {}",user.username, user.email, user.active, user.sign_in_count)
}