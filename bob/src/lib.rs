pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();

    if trimmed.is_empty() {
        return "Fine. Be that way!";
    }

    let has_letters = trimmed.chars().any(|c| c.is_alphabetic());

    let is_yelling = has_letters && trimmed.chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| c.is_uppercase());

    let is_question = trimmed.ends_with('?');

    if is_yelling && is_question {
        "Calm down, I know what I'm doing!"
    } else if is_yelling {
        "Whoa, chill out!"
    } else if is_question {
        "Sure."
    } else {
        "Whatever."
    }
}
