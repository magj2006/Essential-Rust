pub fn reply(message: &str) -> &str {
    let is_yelling = |m: &str| m.chars().any(|c| c.is_ascii_alphabetic()) && m.to_uppercase() == m;

    match message.trim_end() {
        m if m.is_empty() => "Fine. Be that way!",
        m if m.ends_with('?') && is_yelling(m) => "Calm down, I know what I'm doing!",
        m if m.ends_with('?') => "Sure.",
        m if is_yelling(m) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
