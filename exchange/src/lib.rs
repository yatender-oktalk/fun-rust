
// #[derive(PartialEq, Debug)]
pub struct GBP(i32);

pub fn money_pointer(i: i32) -> &GBP {
    let g = GBP(i);
    &g
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let g = money_pointer(3);
        assert_eq!(*g, GBP(3));
    }
}
