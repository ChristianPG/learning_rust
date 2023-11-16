enum IpAddressKind {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

// #[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // Example of Enums using IP addresses
    let home = IpAddressKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddressKind::V6(String::from("::1"));

    // Examples of the Option built-in enum
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    

}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn verify_quarter(coin: Coin) {
    if let Coin::Quarter(state) = coin {
        println!("This is a shiny quarter from {:?}", state);
    }
}
