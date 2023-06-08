#[derive(Debug)]
enum IpAddr{
    V4(u8,u8,u8,u8),
    V6(String),
}

#[derive(Debug)]
enum UsState {
    SouthCarolina,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));
    route(home);
    route(loopback);

    let coin1 = Coin::Penny;
    let coin2 = Coin::Nickel;
    let coin3 = Coin::Dime;
    let coin4 = Coin::Quarter(UsState::SouthCarolina);

    value_in_cents(coin1);
    value_in_cents(coin2);
    value_in_cents(coin3);
    value_in_cents(coin4);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    print!("{:?} plus one is {:?}. {:?} is nothing.", five,six,none);
}

fn route(ip_kind: IpAddr) {
    println!("{:#?}",ip_kind);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}",state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}
