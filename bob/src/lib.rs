pub fn reply(message: &str) -> &str {
    let msg = message.trim();
    if msg.is_empty() {
        return "Fine. Be that way!";
    }

    let is_question = msg.ends_with('?');
    let is_yelling = msg.contains(char::is_alphabetic) && msg == msg.to_uppercase();

    match (is_question, is_yelling) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        (false, false) => "Whatever.",
    }
}
