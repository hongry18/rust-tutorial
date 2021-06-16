use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();
    let l_word = word.to_lowercase();
    let mut v_word = l_word.chars().collect::<Vec<char>>();
    v_word.sort_unstable();
    let c_word = v_word.into_iter().collect::<String>();

    for &anagram in possible_anagrams {
        let l_anagram = anagram.to_lowercase();
        if l_anagram == l_word {
            continue;
        }

        if l_anagram.len() != l_word.len() {
            continue;
        }

        let mut v_anagram = l_anagram.chars().collect::<Vec<char>>();
        v_anagram.sort_unstable();
        if c_word != v_anagram.into_iter().collect::<String>() {
            continue;
        }

        result.insert(anagram);
    }

    result
}
