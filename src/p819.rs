
#![allow(dead_code)]
use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let bad_chars = ['!','?','\'',',',';','.',' '];

        let mut count = HashMap::new();
        paragraph
            .split(bad_chars)
            .filter(|word| !word.is_empty())
            .map(|word| word
                .to_lowercase()
                .chars().filter(|c| !bad_chars.contains(c)).collect()
            )
            .for_each(|word| {
                *count.entry(word).or_insert(0) += 1;
            });

            count.iter().for_each(|(word, count)| println!("{} is found {} times", word, count));
            
        return count
            .iter()
            .filter(|(word, _)| !banned.contains(word))
            .reduce(|(result, max_count), (word, count)| if count > max_count {(word,count)} else {(result, max_count)})
            .unwrap().0.to_string();
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn basic() {
        assert_eq!(Solution::most_common_word(String::from("This test, is a test"), vec![]), "test");
    }

    #[test]
    fn basic_banned() {
        assert_eq!(Solution::most_common_word(String::from("This first test is test a test first"), vec!["test".to_string()]), "first");
    }
}