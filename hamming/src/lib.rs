use std::iter::zip;
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    let distance = zip(s1.chars(), s2.chars()).filter(|(x,y)| x != y)
                                       .count();
 
    
    return Some(distance);    
}
