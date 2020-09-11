pub fn encode(source: &str) -> String {
    // Use a peekable iterator for look ahead
    let mut source_iter = source.chars().peekable();
    let mut encoded = String::new();

    // I don't use a "for" loop here because I need to access source_iter inside the loop
    loop {
        match source_iter.next() {
            // End of input
            None => break,
            // One char
            Some(current_char) => {
                // Check if there are following identical chars
                let mut count = 1;
                while source_iter.peek() == Some(&current_char) {
                    count += 1;
                    source_iter.next();
                }
                if count == 1 {
                    // Only one
                    encoded.push(current_char);
                } else {
                    // More than one
                    encoded.push_str(&format!("{}{}", count, current_char));
                }
            }
        }
    }

    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut source_iter = source.chars();

    // No "for" loop because we need to access source_iter inside the loop
    while let Some(c) = source_iter.next() {
        if c.is_digit(10) {
            // Found a digit, loop until we find all digit & form a number
            let mut count_str = String::from(c);
            while let Some(c) = source_iter.next() {
                if c.is_digit(10) {
                    count_str.push(c);
                } else {
                    // No more digits, apply count to the last character
                    let count: usize = count_str.parse().unwrap();
                    decoded.push_str(&c.to_string().repeat(count));
                    break;
                }
            }
        } else {
            // Found a character that is not a digit
            decoded.push(c);
        }
    }

    decoded
}
