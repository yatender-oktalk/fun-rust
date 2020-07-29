use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("unable to create file {:?}", err),
            },
            other_err => {
                panic!("Problem in opening the file {:?}", other_err);
            }
        },
    };
}
