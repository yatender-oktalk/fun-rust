enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    println!("Hello, world!");
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny =>1 {
            println!("Luckey ");
            1
        },
        Coin::Nickel =>5 {
            println!("Luckey Nickel");
            5
        },
        Coin::Dime =>10 {
            println!("Luckey Dime");
            10
        },
        Coin::Quarter =>25 {
            println!("Luckey Quarter");
            25
        },
    }
}