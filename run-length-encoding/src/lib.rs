fn find_repetition(source: &str) -> usize {
    assert_ne!(source.len(), 0);
    let mut source_iter = source.chars();
    let c = source_iter.next().unwrap();
    let mut count = 1;
    for next_c in source_iter {
        if next_c == c {
            count += 1;
        } else {
            break;
        }
    }
    count
}

pub fn encode(source: &str) -> String {
    let mut source_iter = source.chars().peekable();
    let mut encoded = String::new();
    loop {
        match source_iter.next() {
            None => break,
            Some(current_char) => {
                let mut count = 1;
                while source_iter.peek() == Some(&current_char) {
                    count += 1;
                    source_iter.next();
                }
                if count == 1 {
                    encoded.push(current_char);
                } else {
                    encoded.push_str(&format!("{}{}", count, current_char));
                }
            }
        }
    }
    encoded
}

pub fn decode(source: &str) -> String {
    unimplemented!("Return the run-length decoding of {}.", source);
}
