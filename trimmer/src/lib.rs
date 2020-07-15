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
    #[test]
    fn it_does_not_work() {
        assert_eq!(trim_left("   hello"), " hello");
    }
}
