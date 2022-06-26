pub fn run(input: &str) -> (i64, i64) {
    _ = input;
    (0, 0)
}

#[cfg(test)] 
pub mod tests {
    #[test]
    fn check_sample() {
        assert_eq!(super::run(include_str!("../input/23-sample.txt")), (0, 0));
    }
    #[test]
    fn check_input() {
        assert_eq!(super::run(include_str!("../input/23.txt")), (0, 0));
    }
}
