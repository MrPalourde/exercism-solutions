use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&'a str> = HashSet::new();
    for i in 0..possible_anagrams.len() {
        if is_anagram(word, possible_anagrams.get(i).unwrap()) {
            anagrams.insert(possible_anagrams.get(i).unwrap());
        }
    }
    return anagrams;
}

fn is_anagram(word1: &str, word2: &str) -> bool {
    if word1.to_lowercase() == word2.to_lowercase() {
        return false;
    } else if word1.chars().count() != word2.chars().count() {
        return false;
    }
    let mut vec = Vec::new();
    for i in 0..word1.chars().count() {
        vec.push(false);
    }
    for i in 0..word1.chars().count() {
        let mut is_in_word = false;
        for j in 0..word2.chars().count() {
            if !vec[j] && word1.chars().nth(i).unwrap().to_string().as_str().to_lowercase() == word2.chars().nth(j).unwrap().to_string().as_str().to_lowercase() {
                is_in_word = true;
                vec[j] = true;
                break;
            }
        }
        if is_in_word == false {
            return false;
        }
    }
    return true;
}
