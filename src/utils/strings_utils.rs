pub fn replace_non_alphanumeric(input: &str) -> String {
    let mut result = String::new();
    let mut prev_char_was_dash = false;

    for c in input.chars() {
        if c.is_alphanumeric() {
            result.push(c);
            prev_char_was_dash = false;
        } else if !prev_char_was_dash {
            result.push('-');
            prev_char_was_dash = true;
        }
    }

    result
}
pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
