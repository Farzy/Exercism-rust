pub fn reply(message: &str) -> &str {
    let msg = message.trim();
    if msg.is_empty() {
        return "Fine. Be that way!";
    }

    let is_question = msg.chars().last().unwrap() == '?';
    let is_yelling = msg.chars().any(|c| c.is_ascii_alphabetic())
        && msg
            .chars()
            .all(|c| !c.is_ascii_alphabetic() || c.is_ascii_uppercase());

    match (is_question, is_yelling) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        (false, false) => "Whatever.",
    }
}
