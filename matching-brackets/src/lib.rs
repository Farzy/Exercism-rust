pub fn brackets_are_balanced(string: &str) -> bool {
    let mut paren_cnt = 0;
    let mut curly_cnt = 0;
    let mut bracket_cnt = 0;

    for c in string.chars() {
        match c {
            '(' => paren_cnt += 1,
            ')' => paren_cnt -= 1,
            '{' => curly_cnt += 1,
            '}' => curly_cnt -= 1,
            '[' => bracket_cnt += 1,
            ']' => bracket_cnt -= 1,
            _ => (),
        }
        if paren_cnt < 0 || curly_cnt < 0 || bracket_cnt < 0 {
            return false;
        }
    }
    if paren_cnt != 0 || curly_cnt != 0 || bracket_cnt != 0 {
        return false;
    }
    true
}
