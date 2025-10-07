pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec![];
    }

    if len > digits.len() {
        return vec![];
    }

    (0..=digits.len() - len)
        .map(|i| digits[i..i + len].to_string())
        .collect()
}