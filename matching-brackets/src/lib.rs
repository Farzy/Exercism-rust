use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    // Keep track of open brackets in order
    let mut open_brackets: Vec<char> = vec![];
    // Helper for matching closing brackets to the opening one
    let mut opposite_brackets: HashMap<char, char> = HashMap::new();
    opposite_brackets.insert(')', '(');
    opposite_brackets.insert('}', '{');
    opposite_brackets.insert(']', '[');

    for current_char in string.chars() {
        match current_char {
            '(' | '{' | '[' => open_brackets.push(current_char),
            ')' | '}' | ']' => {
                if let Some(last_open_bracket) = open_brackets.pop() {
                    if opposite_brackets.get(&current_char).unwrap() != &last_open_bracket {
                        // Current closing bracket does not match last opened bracket kind
                        return false;
                    }
                } else {
                    // There was no open bracket of any kind
                    return false;
                }
            }
            _ => (),
        }
    }
    // Check for unbalanced brackets at end of string
    open_brackets.len() == 0
}
