pub fn reply(message: &str) -> &str {
    let trimmed_message = message.trim();
    let is_question: bool = trimmed_message.ends_with('?');
    let is_all_uppercase: bool = trimmed_message == trimmed_message.to_uppercase() &&
        trimmed_message.chars().any(|c| c.is_alphabetic());

    if is_question && is_all_uppercase {
        return "Calm down, I know what I'm doing!";
    } else if is_question {
        return "Sure.";
    } else if is_all_uppercase {
        return "Whoa, chill out!";
    } else if trimmed_message.is_empty(){
        return "Fine. Be that way!";
    }
    "Whatever."
}
