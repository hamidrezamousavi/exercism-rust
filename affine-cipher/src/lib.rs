use gcd::Gcd;

const M:i32 = 26;
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

fn is_coprim(a: i32, m: i32) -> bool {
    (a as u32).gcd(m as u32) == 1
}

fn encode_letter(letter: &char, a: i32, b: i32) -> char {
    if letter.is_ascii_digit(){ return *letter;}
    char::from_u32(((((*letter as i32 - 97) * a + b) % M) + 97) as u32).unwrap()
}

fn get_mmi(a: i32, m: i32) -> Option<i32> {
    
    (1..m).into_iter().find(|x| (a * x)% m == 1)
    
}
fn decode_letter(letter: &char, a: i32, b: i32) -> char {
    if letter.is_ascii_digit(){ return *letter;}
    
    let mmi = get_mmi(a, M).unwrap();
    char::from_u32(((((*letter as i32 - 97 - b) * mmi).rem_euclid(M)) + 97) as u32)
        .unwrap()
}

pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !is_coprim(a, M) {
        return Err(AffineCipherError::NotCoprime(a));
    }
    Ok(
    plaintext
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| encode_letter(&c, a, b))
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
    )
}

pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !is_coprim(a, M) {
        return Err(AffineCipherError::NotCoprime(a));
    }
    Ok(
       ciphertext.chars()
                 .filter(|c| c.is_alphanumeric())
                 .map(|c| decode_letter(&c,a,b))
                 .collect::<String>()  
    )
    /*   
    let mut decoded_text = String::new();
    
    for letter in ciphertext.chars() {
        if letter.is_ascii_alphanumeric() {
            decoded_text.push(decode_letter(&letter, a, b));
        }
     
    }

    Ok(decoded_text)
*/
}
