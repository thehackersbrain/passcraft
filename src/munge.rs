use crate::config::CHAR_MAP;

pub fn leet_speak(words: Vec<String>) -> Vec<String> {
    let mut leet_words = Vec::new();

    for i in words {
        let leet_word: String = i
            .chars()
            .map(|c| CHAR_MAP.get(&c).copied().unwrap_or(c))
            .collect();

        leet_words.push(leet_word);
    }

    leet_words
}
