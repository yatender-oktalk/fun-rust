use std::fs::File;
use std::io;

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

// fn main() {
//     let f = File::open("hello.txt");
//     let _f = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("{:?}", error);
//             })
//         } else {
//             panic!("problem in opening in file {:?}", error)
//         }
//     });
// }

fn main() {
    let x = read_username_from_file();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Error(err) => return Err(err),
    };

    let mut e = String::new();

    match f.read_to_string(&mut String) {
        Ok(_) => s,
        Err(e) => Err(e),
    }
}
