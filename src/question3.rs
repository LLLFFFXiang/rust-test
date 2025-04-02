// 请从命令行读取一行字符串（例如 "apple banana pear banana apple banana"）。
// 使用空格进行拆分，统计每个单词出现的次数，并以从高到底的顺序输出。
// 如果出现次数相同，按单词本身的字典序从小到大排序输出。
use std::collections::HashMap;

pub fn count_words(input: &str) -> HashMap<String, u32> {
    let mut word_counts = HashMap::new();
    for word in input.split_whitespace() {
        *word_counts.entry(word.to_string()).or_insert(0) += 1;
    }
    word_counts
}

pub fn sort_word_counts(word_counts: HashMap<String, u32>) -> Vec<(String, u32)> {
    let mut count_vec: Vec<_> = word_counts.into_iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    count_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let counts = count_words("");
        assert_eq!(counts.len(), 0);
    }

    #[test]
    fn test_basic_counting() {
        let counts = count_words("hello world hello");
        assert_eq!(counts["hello"], 2);
        assert_eq!(counts["world"], 1);
    }

    #[test]
    fn test_case_sensitive() {
        let counts = count_words("Hello hello HELLO");
        assert_eq!(counts["Hello"], 1);
        assert_eq!(counts["hello"], 1);
        assert_eq!(counts["HELLO"], 1);
    }

    #[test]
    fn test_sorting_logic() {
        let mut counts = HashMap::new();
        counts.insert("banana".to_string(), 3);
        counts.insert("apple".to_string(), 2);
        counts.insert("pear".to_string(), 3);  // 与banana同次数
        
        let sorted = sort_word_counts(counts);
        assert_eq!(sorted[0].0, "banana");  // 次数最多
        assert_eq!(sorted[1].0, "pear");    // 同次数按字母序
        assert_eq!(sorted[2].0, "apple");
    }

    #[test]
    fn test_multiple_spaces() {
        let counts = count_words("  multiple   spaces  between  words  ");
        assert_eq!(counts["multiple"], 1);
        assert_eq!(counts["spaces"], 1);
        assert_eq!(counts["between"], 1);
        assert_eq!(counts["words"], 1);
    }
}