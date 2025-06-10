use core::panic;
use std::borrow::Cow;

// The function evaluates a statement in reverse Polish notation
pub fn evaluate_formula(expr: &str) -> Result<bool, Cow<'static, str>> {
    // Stack to store operands
    let mut stack: Vec<bool> = Vec::new();
    // Looping over the string
    for (i, ch) in expr.char_indices() {
        // If operand, store in the stack
        if "01".contains(ch) {
            stack.push(ch == '1');
        } 
        // Else (if operator), attempt evaluation of operands
        else if "!&|^>=".contains(ch) {
            // If stack's size is less than 2, error
            if stack.len() < 2 || ch == '!' && stack.len() < 1 {
                return Err(format!("Not enough operands for the operator `{}` at the position {}", ch, i).into());
            }
            else {
                let op_1 = stack.pop().unwrap();
                if ch == '!' {
                    stack.push(!op_1);
                } else {
                    let op_2 = stack.pop().unwrap();
                    let result = match ch {
                        '&' => op_1 && op_2,
                        '|' => op_1 || op_2,
                        '^' => op_1 ^ op_2,
                        '>' => !(op_1 && !op_2),
                        '=' => op_1 == op_2,
                        _ => panic!("Impossible to process operator `{}` here", ch)
                    };
                    stack.push(result);
                }
            }
        }
        else {
            return Err("Input contains wrong characters".into());
        }
    }
    if stack.len() != 1 {
        return Err("Too many operators".into());
    }
    Ok(stack.pop().unwrap())
}


#[cfg(test)]
mod tests {
    use super::evaluate_formula;

    #[test]
    fn evaluate_formula_examples_from_exercise() {
        assert_eq!(evaluate_formula("10&").unwrap(), false);
        assert_eq!(evaluate_formula("10|").unwrap(), true);
        assert_eq!(evaluate_formula("11>").unwrap(), true);
        assert_eq!(evaluate_formula("10=").unwrap(), false);
        assert_eq!(evaluate_formula("1011||=").unwrap(), true);
    }
}
