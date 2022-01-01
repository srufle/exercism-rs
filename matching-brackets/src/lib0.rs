// fn is_balanced(vec: &mut Vec<char>, closing_char: char) -> bool {
fn is_balanced(vec: &mut Vec<char>, closing_char: char) -> &mut Vec<char> {
    let last = vec.last();

    vec.pop();

    match last {
        Some(c) if closing_char == ']' => {
            println!("Some(c) if closing_char == ']'  {}", c);
            if c.to_owned() == '(' || c.to_owned() == '{' {
                println!("false");
                vec
            } else {
                println!("true");
                vec
            }
        }
        Some(c) if closing_char == '}' => {
            println!("Some({0}) if closing_char == 'curly' {1}", c, closing_char);
            if c.to_owned() == '(' || c.to_owned() == '[' {
                println!("false");
                vec
            } else {
                println!("true");
                vec
            }
        }
        Some(c) if closing_char == ')' => {
            println!("Some(c) if closing_char == ')' {}", c);
            if c.to_owned() == '{' || c.to_owned() == '[' {
                println!("false");
                vec
            } else {
                println!("true");
                vec
            }
        }
        // None => false,
        _ => vec,
    }
}
fn is_balanced_v0(vec: &mut Vec<char>, closing_char: char) -> bool {
    let last = vec.last();

    vec.pop();

    match last {
        Some(c) if closing_char == ']' => {
            println!("Some(c) if closing_char == ']'  {}", c);
            if c.to_owned() == '(' || c.to_owned() == '{' {
                println!("false");
                false
            } else {
                println!("true");
                true
            }
        }
        Some(c) if closing_char == '}' => {
            println!("Some({0}) if closing_char == 'curly' {1}", c, closing_char);
            if c.to_owned() == '(' || c.to_owned() == '[' {
                println!("false");
                false
            } else {
                println!("true");
                true
            }
        }
        Some(c) if closing_char == ')' => {
            println!("Some(c) if closing_char == ')' {}", c);
            if c.to_owned() == '{' || c.to_owned() == '[' {
                println!("false");
                false
            } else {
                println!("true");
                true
            }
        }
        // None => false,
        _ => false,
    }
}
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut v: &mut Vec<char> = &mut Vec::new();

    let mut balanced = false;
    if !string.is_empty() {
        for c in string.chars() {
            println!("{}", c);
            match c {
                '[' | '{' | '(' => v.push(c),
                _ => (),
            }
            if v.is_empty() {
                return false;
            }
            v = match c {
                ']' => is_balanced(&mut v, c),

                '}' => is_balanced(&mut v, c),

                ')' => is_balanced(&mut v, c),

                _ => &mut v,
            }
        }
    }

    v.is_empty()
}
