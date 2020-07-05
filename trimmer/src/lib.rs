pub fn trim_left(s: &str) -> &str {
    for (i, c) in s.char_indices() {
        if c == ' ' {
            continue;
        }
        return s.get(i..s.len()).unwrap();
    }
    ""
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(trim_left("   hello"), "hello");
    }
}
