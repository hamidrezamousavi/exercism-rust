use std::collections::HashMap;
fn remove_apostrophes(word:&str)-> String{
    let mut begin = 0;
    let mut end = word.len();
    if word.starts_with("\'"){
        begin += 1;
    }
    if word.ends_with("\'"){
        end -= 1;
    }
    if begin >= end {
        return "".to_string();
    }
    return word[begin..end].to_string();
  

}
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut words_count = words.chars()
                          .map(|c| match c{
                                        n if  n.is_ascii_punctuation() && c != '\'' => ' ',
                                        _ => c.to_ascii_lowercase(), })
                          .collect::<String>()
                          .split_whitespace()
                          .map(|word|  remove_apostrophes(word))
                          .filter(|word| !word.is_empty())
                          .fold(HashMap::new(),|mut map, word|{
                              *map.entry(word).or_insert(0)+=1;
                              map});                  
                          

    
    words_count
}
