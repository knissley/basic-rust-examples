// This problem really shows the power of match statements

pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];
    for char in string.chars() {
        match char {
            ')' => match stack.pop() {
                Some(char) => {
                    if char != '(' {
                        return false;
                    }
                }
                None => return false,
            },
            ']' => match stack.pop() {
                Some(char) => {
                    if char != '[' {
                        return false;
                    }
                }
                None => return false,
            },
            '}' => match stack.pop() {
                Some(char) => {
                    if char != '{' {
                        return false;
                    }
                }
                None => return false,
            },
            _ => stack.push(char),
        }
    }
    stack.is_empty()
}
