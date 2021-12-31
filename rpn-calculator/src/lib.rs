#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];
    use CalculatorInput::*;
    for input in inputs {
        match input {
            Add => {
                if let Some(b) = stack.pop() {
                    if let Some(a) = stack.pop() {
                        stack.push(a + b);
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            Subtract => {
                if let Some(b) = stack.pop() {
                    if let Some(a) = stack.pop() {
                        stack.push(a - b);
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            Multiply => {
                if let Some(b) = stack.pop() {
                    if let Some(a) = stack.pop() {
                        stack.push(a * b);
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            Divide => {
                if let Some(b) = stack.pop() {
                    if let Some(a) = stack.pop() {
                        stack.push(a / b);
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            Value(value) => stack.push(*value),
        }
    }
    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}
