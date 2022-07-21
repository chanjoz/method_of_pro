use std::collections::HashMap;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn check_sub_string (source: &[String], target: &[String]) -> String {
    let mut word_candi = HashMap::<&String, usize>::new();
    for w in target {
        *word_candi.entry(w).or_insert(0) += 1;
    }
    let mut word_window = HashMap::<&String, usize>::new();
    let mut match_cnt = 0 as usize;
    let need_cnt = word_candi.len();
    let mut left = 0 as usize;
    let mut right = 0 as usize;
    let mut start = 0 as usize;
    let mut min_len = usize::MAX;

    while right < source.len() {
        let word = &source[right];
        if word_candi.contains_key(word) {
            *word_window.entry(word).or_insert(0) += 1;
            if word_window[word] == word_candi[word] {
                match_cnt += 1;
            }
        }
        right += 1;

        while match_cnt == need_cnt {
            if right-left < min_len {
                start = left;
                min_len = right-left;
            }
            let d = &source[left];
            if word_window.contains_key(d) {
                if word_window[d] == word_candi[d] {
                    match_cnt -= 1;
                }
                if let Some(t) = word_window.get_mut(d) {
                    *t -= 1;
                }
            }
            left += 1;
        }
    }
    if min_len == usize::MAX {
        return "".to_string();
    }
    
    let res = &source[start..(start+min_len)];
    res.join(" ")    
}

fn main() -> io::Result<()> {
    let mut source = Vec::new();
    let f = File::open("beam_birth.txt")?;
    let f = BufReader::new(f);
    for l in f.lines() {
        l.unwrap().split(" ").fold(&mut source, |acc, w|{ 
            acc.push(w.to_string());
            acc
        });
    }
    println!("{}", check_sub_string(&source, &["hello".to_string(), "world".to_string(), "beam".to_string()]));
    Ok(())
    
}