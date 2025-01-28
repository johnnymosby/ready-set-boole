// The task of the exercise is to write a function,
// which takes two positive numbers and returns their sum.
// Since there may be two definitions of positive numbers (with or without 0),
// I could choose the one with less limitations: with 0.

pub fn add(mut a: u32, mut b: u32) -> u32 { 
    let mut carry: u32;
    while b != 0 {
        carry = (a & b) << 1;
        a = a ^ b;
        b = carry;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_small_positive_numbers() {
        let a = 2;
        let b = 10;
        let result = add(a, b);
        assert_eq!(result, a + b);
    }

    #[test]
    fn add_null_and_positive() {
        let a = 0;
        let b = 481;
        let result = add(a, b);
        assert_eq!(result, a + b);
    }

    #[test]
    fn add_null_and_max_unsigned() {
        let a = u32::MAX;
        let b = 0;
        let result = add(a, b);
        assert_eq!(result, a + b);
    }

    // #[test]
    // #[should_panic]
    // fn add_with_overflow() {
    //     let a = 1;
    //     let b = u32::MAX;
    //     let result = add(a, b);
    //     assert_eq!(result, a + b);
    // }
}
