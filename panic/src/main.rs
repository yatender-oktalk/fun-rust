use std::fs::File;
use std::io::ErrorKind;

// fn main() {
//     let f = File::open("hello.txt");
//     let _f = match f {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(err) => panic!("unable to create file {:?}", err),
//             },
//             other_err => {
//                 panic!("Problem in opening the file {:?}", other_err);
//             }
//         },
//     };
// }

fn main() {
    let f = File::open("hello.txt");
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("{:?}", error);
            })
        } else {
            panic!("problem in opening in file {:?}", error)
        }
    });
}
