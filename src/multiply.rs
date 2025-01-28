pub fn multiply(mut a: u32, mut b: u32) -> u32 {
    let mut result: u32 = 0;
    while b != 0 {
        if (b & 1) == 1 {
            result = crate::add::add(a, result);
        }
        a = a << 1;
        b = b >> 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply_small_numbers() {
        let a: u32 = 13;
        let b: u32 = 534;
        let result = multiply(a, b);
        assert_eq!(result, a * b);
    }

    #[test]
    fn multiply_null_with_a_number() {
        let a: u32 = 0;
        let b: u32 = 534;
        let result = multiply(a, b);
        assert_eq!(result, a * b);
    }

    #[test]
    fn multiply_null_with_a_null() {
        let a: u32 = 0;
        let b: u32 = 534;
        let result = multiply(a, b);
        assert_eq!(result, a * b);
    }

    #[test]
    fn multiply_null_with_max_unsigned() {
        let a: u32 = u32::MAX;
        let b: u32 = 0;
        let result = multiply(a, b);
        assert_eq!(result, a * b);
    }

    #[test]
    fn multiply_one_with_max_unsigned() {
        let a: u32 = u32::MAX;
        let b: u32 = 1;
        let result = multiply(a, b);
        assert_eq!(result, a * b);
    }

    // #[test]
    // #[should_panic]
    // fn multiply_with_overflow() {
    //     let a = 2;
    //     let b = u32::MAX;
    //     let result = multiply(a, b);
    //     assert_eq!(result, a + b);
    // }
}
