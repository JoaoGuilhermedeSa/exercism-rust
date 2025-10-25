pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    fn article_for(word: &str) -> &'static str {
        match word.chars().next() {
            Some(c) if matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u') => "an",
            _ => "a",
        }
    }

    let mut result = String::new();

    for pair in list.windows(2) {
        let (first, second) = (pair[0], pair[1]);
        let article = article_for(first);
        result.push_str(&format!("For want of {} {} the {} was lost.\n", article, first, second));
    }

    let article_first = article_for(list[0]);
    result.push_str(&format!("And all for the want of {} {}.", article_first, list[0]));

    result
}
