mod sound{
    pub mod instrument {
        pub mod woodwind {
            pub fn clarinet() {
                println!("clarinet goes here")
            }
        }

        pub fn guitar() {
            println!("guitar goes here")
        }
    }

    pub mod voice {
        pub fn guitar() {
            println!("voice guitar goes here")
        }
    }

}

fn main() {
    println!("Hello, world!");
    // absolute path
    crate::sound::instrument::woodwind::clarinet();
    crate::sound::instrument::guitar();
    sound::instrument::woodwind::clarinet();
    sound::voice::guitar();
}
