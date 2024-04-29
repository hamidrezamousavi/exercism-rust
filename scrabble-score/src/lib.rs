/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.chars()
        .map(|x| letter_score(x)as u64)
        .sum()
}

fn letter_score(letter:char)->u8{
    let letter = letter.to_ascii_uppercase();
    
    match letter {
        'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
        'D' | 'G' => 2,     
        'B' | 'C' | 'M' | 'P' => 3,     
        'F' | 'H' | 'V' | 'W' | 'Y' => 4,     
        'K'             => 5,     
        'J' | 'X'          => 8,     
        'Q' | 'Z'          => 10, 
        _ => 0,   
    }
    
}
