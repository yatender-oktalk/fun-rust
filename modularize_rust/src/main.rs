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
}
