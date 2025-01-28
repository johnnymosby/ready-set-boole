pub fn convert_to_gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

#[cfg(test)]
mod tests {
    use super::convert_to_gray_code;

    #[test]
    fn convert_examples_from_exercise() {
        assert_eq!(convert_to_gray_code(0), 0);
        assert_eq!(convert_to_gray_code(1), 1);
        assert_eq!(convert_to_gray_code(2), 3);
        assert_eq!(convert_to_gray_code(3), 2);
        assert_eq!(convert_to_gray_code(4), 6);
        assert_eq!(convert_to_gray_code(5), 7);
        assert_eq!(convert_to_gray_code(6), 5);
        assert_eq!(convert_to_gray_code(7), 4);
        assert_eq!(convert_to_gray_code(8), 12);
    }
}
