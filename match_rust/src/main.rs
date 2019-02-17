enum UsState {
    Alabama,
    Alaska,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    println!("Hello, world!");
    let coin = Coin::Quarter;
    let value = value_in_cents(coin);
    println!("{}", value);
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        },
        Coin::Nickel => {
            println!("Lucky Nickel");
            5
        },
        Coin::Dime => {
            println!("Lucky Dime");
            10
        },
        Coin::Quarter => {
            println!("Lucky Quarter");
            25
        },
    }
}