use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();

    let word_lower = word.to_lowercase();
    let mut word_sorted: Vec<char> = word_lower.chars().collect();
    word_sorted.sort();

    for &candidate in possible_anagrams {
        let candidate_lower = candidate.to_lowercase();


        if candidate_lower == word_lower {
            continue;
        }

        let mut candidate_sorted: Vec<char> = candidate_lower.chars().collect();
        candidate_sorted.sort();

        if candidate_sorted == word_sorted {
            result.insert(candidate);
        }
    }

    return result
}
