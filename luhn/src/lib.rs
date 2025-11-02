pub fn is_valid(code: &str) -> bool {
    if code
        .chars()
        .any(|ch| ch != ' ' && !ch.is_ascii_digit())
    {
        return false;
    }

    let mut sum: u32 = 0;
    let mut digits = 0usize;

    for (i, ch) in code.chars().rev().filter(|&c| c != ' ').enumerate() {
        let mut d = ch.to_digit(10).unwrap();
        if i % 2 == 1 {
            d *= 2;
            if d > 9 {
                d -= 9;
            }
        }
        sum += d;
        digits += 1;
    }

    digits > 1 && sum % 10 == 0
}