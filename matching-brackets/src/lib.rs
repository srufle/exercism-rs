// Insired By
// https://exercism.org/tracks/rust/exercises/matching-brackets/solutions/michaelmez39
// I knew it had ro use a stack, but got too fancy my first try
// still had failures:
//     complex_latex_expression
//     math_expression
//     paired_and_nested_brackets
//     paired_square_brackets
//     paired_with_whitespace
//     several_paired_brackets
//     simple_nested_brackets
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut my_stack: Vec<char> = Vec::new();
    for c in string.chars() {
        match c {
            '[' | '{' | '(' => my_stack.push(c),
            ']' => {
                if my_stack.pop() != Some('[') {
                    return false;
                }
            }

            '}' => {
                if my_stack.pop() != Some('{') {
                    return false;
                }
            }

            ')' => {
                if my_stack.pop() != Some('(') {
                    return false;
                }
            }
            _ => (),
        }
    }
    return my_stack.is_empty();
}
