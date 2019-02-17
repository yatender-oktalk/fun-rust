use std::collections;

mod sound{

    pub mod instrument {
        pub mod woodwind {
            pub fn clarinet() -> () {
                super::super::super::breathe_in()
            }
        }

        pub fn guitar() -> () {
            println!("guitar goes here")
        }
    }

    pub mod voice {
        pub fn guitar() -> () {
            println!("voice guitar goes here")
        }
    }

}

use sound::instrument::woodwind;
use rand::Rng;
// nested path for dependencies
use std::{cmp::Ordering, io};

fn breathe_in() -> (){
    println!("breathe_in goes here")
}

fn main() {
    println!("Hello, world!");
    // absolute path
    crate::sound::instrument::woodwind::clarinet();
    crate::sound::instrument::guitar();
    woodwind::clarinet();
    sound::voice::guitar();

    let mut map = collections::HashMap::new();
    map.insert("1",2);
    map.insert("items",2);

    println!("{:?}", map);
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("{}", secret_number);
}
