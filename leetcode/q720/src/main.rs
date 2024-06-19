#![allow(unused)]

pub fn longest_word1(mut words: Vec<String>) -> String {
    // let mut words = words.clone();
    // words.sort();
    // words.sort_by(|a, b| b.len().cmp(&a.len()));
    words.sort_by(|a, b| b.len().cmp(&a.len()).then(a.cmp(b)));


    let mut result = String::new();

    for word in words.iter() {
        let word2 = find_one_char_shorter_recursive(word, &words);
        if word2.is_some() {
            return word.to_string();
            // break;
        }
    }

    fn find_one_char_shorter_recursive<'a>(word: &'a str, words: &'a Vec<String>) -> Option<&'a str> {
        if word.len() == 1 {
            return Some(word);
        }

        let w = find_one_char_shorter(word, words);
        if w.is_some() {
            return find_one_char_shorter_recursive(w.unwrap(), words);
        } else {
            return None;
        }
    }

    fn find_one_char_shorter<'a>(word: &'a str, words: &'a Vec<String>) -> Option<&'a str> {
        for w in words.iter() {
            if w.len() < word.len() - 1 {
                break;
            }
            if w.len() == word.len() - 1 {
                if word.starts_with(w) {
                    return Some(w.as_str());
                }
            }
        }
        None
    }

    "".to_string()
}

pub fn longest_word2(words: Vec<String>) -> String {
    let mut words = words.clone();
    let longest_word = words.iter().max_by_key(|s| s.len()).unwrap();

    let mut tmp: Vec<Vec<String>> = vec![vec![]; longest_word.len()];

    for word in words.iter() {
        tmp[word.len()-1].push(word.to_string());
    }

    // println!("{:?}", tmp);

    for i in 0..tmp.len()-1 {
        let mut retain2 = vec![];
        for w2 in tmp[i+1].iter() {
            for w1 in tmp[i].iter() {
                if w2.starts_with(w1) {
                    retain2.push(w2.to_string());
                    break;
                }
            }
            // println!("retain: {:?}", retain);
        }
        tmp[i+1].retain(|s| retain2.contains(s));
        tmp[i+1].sort();
    }

    // println!("aaaa {:?}", tmp);
    let first_non_empty = tmp.iter().rev().find(|v| !v.is_empty());
    if let Some(non_empty_vec) = first_non_empty {
        return non_empty_vec[0].to_string();
    }

   "".to_string()
}

pub fn longest_word(mut words: Vec<String>) -> String {
    // let mut words = words.clone();
    // words.sort();
    // words.sort_by(|a, b| b.len().cmp(&a.len()));
    words.sort_by(|a, b| b.len().cmp(&a.len()).then(a.cmp(b)));

    for word in words.iter() {
        let mut found = true;
        for i in 1..word.len() {
            if !words.contains(&word[..i].to_string()) {
                found = false;
                break;
            }
        }
        if found {
            return word.to_string();
        }
    }
    "".to_string()
}

pub fn longest_word4(words: Vec<String>) -> String {

    let mut words = words;
    words.sort_by(|a, b| b.len().cmp(&a.len()).then(a.cmp(b)));

    let word_set: HashSet<_> = words.iter().collect();

    for word in words.iter() {
        let mut found = true;
        for i in 1..word.len() {
            if !word_set.contains(&word[..i].to_string()) {
                found = false;
                break;
            }
        }
        if found {
            return word.to_string();
        }
    }
    "".to_string()
}

pub fn longest_word5(words: Vec<String>) -> String {
    let word_set: HashSet<_> = words.iter().map(|s| s.as_str()).collect();
    let mut longest: &String = &String::new();

    for word in &words {
        let mut valid = true;
        for i in 1..word.len() {
            if !word_set.contains(&word[..i]) {
                valid = false;
                break;
            }
        }
        if valid && (word.len() > longest.len() || (word.len() == longest.len() && word < longest)) {
            longest = word;
        }
    }

    longest.to_string()
}

use std::cmp::Ordering::{Less, Equal, Greater};

    pub fn longest_word6(mut words: Vec<String>) -> String {
        words.sort_unstable_by(|left,right|{
            if left.len() < right.len() {
                return Less;
            }
        
            if left.len() == right.len() {
                if left > right {
                    return Less;
                } else {
                    return Greater;
                }
            }
            
            Greater
        });

        words.reverse();

        for i in 0..words.len() {
            let candidate = &words[i];

            for j in 0..candidate.len() {
                let snippet = candidate[..=j].to_string();
                if !words.contains(&snippet) {
                    break;
                }

                if j == candidate.len() - 1 {
                    return candidate.to_string();
                }
            }
        }

        
        "".to_string()
    }

pub fn longest_word8(words: Vec<String>) -> String {
    let mut words = words;
    words.sort_by(|a, b| b.len().cmp(&a.len()).then(a.cmp(b)));
    let word_set: HashSet<&str> = words.iter().map(|s| s.as_str()).collect();

    for word in &words {
        if is_buildable(word, &word_set) {
            return word.clone();
        }
    }

    "".to_string()
}

fn is_buildable(word: &str, word_set: &HashSet<&str>) -> bool {
    for i in 1..word.len() {
        if !word_set.contains(&word[..i]) {
            return false;
        }
    }
    true
}

pub fn longest_word7(words: Vec<String>) -> String {
    let mut word_set: HashSet<String> = HashSet::new();
    let mut longest_word = String::new();

    // Sort words lexicographically to ensure the order for prefix checks
    let mut sorted_words = words;
    sorted_words.sort();

    for word in &sorted_words {
        // If the word length is 1 or its prefix exists in the set
        if word.len() == 1 || word_set.contains(&word[..word.len() - 1]) {
            // Insert the word into the set
            word_set.insert(word.clone());
            // Update the longest_word if current word is longer or lexicographically smaller if equal length
            if word.len() > longest_word.len() || (word.len() == longest_word.len() && word < &longest_word) {
                longest_word = word.clone();
            }
        }
    }

    longest_word
}

use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let test_cases = vec![
        vec!["w","wo","wor","worl","world"],
        vec!["a", "banana", "app", "appl", "ap", "apply", "apple", "ac", "cd"],
        vec!["company", "compan", "compa", "comp", "com", "co", "c", "compile", 
        "compil", "compi"],
        vec!["a", "banana", "app", "appl", "ap", "apply", "apple", "ac", "cd", "company", "compan", "compa", "comp", "com", "co", "c", "compile", 
        "compil", "compi", "tyfkhlkjhL"],
        vec!["wo","wor","worl","world"],
        vec!["ogz","eyj","e","ey","hmn","v","hm","ogznkb","ogzn","hmnm","eyjuo","vuq","ogznk","og","eyjuoi","d"],
        vec!["w","wo","wor","worl","world", "a", "banana", "app", "appl", "ap",
        "apply", "apple", "ac", "cd", "company", "compan", "compa", "comp", "com",
        "co", "c", "compile", "compil", "compi", "a", "banana", "app", "appl",
        "ap", "apply", "apple", "ac", "cd", "company", "compan", "compa", "comp", "com", "co", "c", "compile", 
        "compil", "compi", "tyfkhlkjhL",
        "wo","wor","worl","world",
        "ogz","eyj","e","ey","hmn","v","hm","ogznkb","ogzn","hmnm","eyjuo","vuq","ogznk","og","eyjuoi","d"],
        vec!["yo","ew","fc","zrc","yodn","fcm","qm","qmo","fcmz","z","ewq","yod","ewqz","y"],

    ];

    let start = Instant::now();
    let mut result = "".to_string();
    for i in 0..1000 {
        for words in test_cases.clone() {
            let words_vec = words.iter().map(|s| s.to_string()).collect();
            result = longest_word1(words_vec);
        }
    }
    let duration = start.elapsed();
    println!("answer = {:?}", result);
    println!("time taken = {:?}", duration);

    let start = Instant::now();
    let mut result = "".to_string();
    for i in 0..1000 {
        for words in test_cases.clone() {
            let words_vec = words.iter().map(|s| s.to_string()).collect();
            result = longest_word2(words_vec);
        }
    }
    let duration = start.elapsed();
    println!("answer = {:?}", result);
    println!("time taken = {:?}", duration);

    let start = Instant::now();
    let mut result = "".to_string();
    for i in 0..1000 {
        for words in test_cases.clone() {
            let words_vec = words.iter().map(|s| s.to_string()).collect();
            result = longest_word(words_vec);
        }
    }
    let duration = start.elapsed();
    println!("answer = {:?}", result);
    println!("time taken = {:?}", duration);

    let start = Instant::now();
    let mut result = "".to_string();
    for i in 0..1000 {
        for words in test_cases.clone() {
            let words_vec = words.iter().map(|s| s.to_string()).collect();
            result = longest_word4(words_vec);
        }
    }
    let duration = start.elapsed();
    println!("answer = {:?}", result);
    println!("time taken = {:?}", duration);

    let start = Instant::now();
    let mut result = "".to_string();
    for i in 0..1000 {
        for words in test_cases.clone() {
            let words_vec = words.iter().map(|s| s.to_string()).collect();
            result = longest_word5(words_vec);
        }
    }
    let duration = start.elapsed();
    println!("answer = {:?}", result);
    println!("time taken = {:?}", duration);

    let start = Instant::now();
    let mut result = "".to_string();
    for i in 0..1000 {
        for words in test_cases.clone() {
            let words_vec = words.iter().map(|s| s.to_string()).collect();
            result = longest_word6(words_vec);
        }
    }
    let duration = start.elapsed();
    println!("answer = {:?}", result);
    println!("time taken = {:?}", duration);

    let start = Instant::now();
    let mut result = "".to_string();
    for i in 0..1000 {
        for words in test_cases.clone() {
            let words_vec = words.iter().map(|s| s.to_string()).collect();
            result = longest_word7(words_vec);
        }
    }
    let duration = start.elapsed();
    println!("answer = {:?}", result);
    println!("time taken = {:?}", duration);
}
