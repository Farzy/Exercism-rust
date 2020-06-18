use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    // Keep track of open brackets in order
    let mut open_brackets: Vec<char> = vec![];
    // Helper for matching closing brackets to the opening one
    let mut opposite_brackets: HashMap<char, char> = HashMap::new();
    opposite_brackets.insert('(', ')');
    opposite_brackets.insert('{', '}');
    opposite_brackets.insert('[', ']');

    for current_char in string.chars() {
        match current_char {
            '(' | '{' | '[' => open_brackets.push(*opposite_brackets.get(&current_char).unwrap()),
            ')' | '}' | ']' => {
                if open_brackets.pop() != Some(current_char) {
                    // Current closing bracket does not match last opened bracket kind
                    // or there is no open bracked
                    return false;
                }
            }
            _ => (),
        }
    }
    // Check for unbalanced brackets at end of string
    open_brackets.is_empty()
}
