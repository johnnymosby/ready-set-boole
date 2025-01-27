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

    // #[test]
    // #[should_panic]
    // fn add_with_overflow() {
    //     let a = 1;
    //     let b = u32::MAX;
    //     let result = add(a, b);
    //     assert_eq!(result, a + b);
    // }
}
