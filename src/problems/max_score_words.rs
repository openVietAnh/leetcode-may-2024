use std::collections::HashMap;

pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut bit = 1;
    let mut count: HashMap<char, i32> = HashMap::new();
    for c in letters {
        count.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }
    while bit <= 1 << words.len() {
        let mut remain = count.clone();
        let mut current_score = 0;
        let mut invalid = false;
        for i in 0..words.len() {
            if invalid {
                break;
            }
            if bit & 1 << i == 1 << i {
                for c in words[i].chars() {
                    if remain.contains_key(&c) {
                        if *remain.get(&c).unwrap() > 0 {
                            *remain.get_mut(&c).unwrap() -= 1;
                            current_score += score[(c as u8 - 'a' as u8) as usize]
                        } else {
                            invalid = true;
                            break;
                        }
                    } else {
                        invalid = true;
                        break;
                    }
                }
            }
        }
        if !invalid {
            result = std::cmp::max(result, current_score);
        }
        bit += 1;
    }
    result
}
