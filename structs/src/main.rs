    #[derive(Debug)]
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//if you're using linux i highly recommend using the mold linker, it improves compile times at least 10x
// - Drew
fn main() {

    // let a = "dog".to_string();
    // {
    //     let b = &a;
    // }
    // print!("A is: {}", a);

    print!("build_user: {:#?}\n", build_user("user1", "user1@example.com"));
    print!("build_user_shorthand: {:#?}\n", build_user_shorthand(String::from("user2"), String::from("user2@example.com")));

    //now with toString
    print!("build_user: {}\n", to_string(build_user("user3", "user3@example.com")));
    print!("build_user_shorthand: {}\n", to_string(build_user_shorthand(String::from("user4"), String::from("user4@example.com"))));
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