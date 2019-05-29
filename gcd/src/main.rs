fn main() {
    let mut m: isize = 8;
    let mut n: isize = 2;

    let resp = calculate_gcd(&mut m, &mut  n);
    println!("{}", resp);

}

fn calculate_gcd(m: &mut isize,  n: &mut isize) -> isize {
    loop {
        let rem = *m % *n;
        *m = *n;
        *n = rem;
        if *n == 0 {
            break *m
        }
    }
}