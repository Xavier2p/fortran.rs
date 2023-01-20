#[cfg(test)]
mod tests {
    // use super::*;
    use test_case::test_case;

    #[test_case(2, 2, 4; "2+2")]
    fn it_works(a: i32, b: i32, c: i32) {
        assert_eq!(a + b, c);
    }
}
