pub fn encode(plain: &str) -> String {
 
    plain.chars()
         .map(|x| x.to_ascii_lowercase())  
         .filter(|x| x.is_alphanumeric())
         .map(|x| encode_char(x))
         .collect::<Vec<_>>()
         .chunks(5)
         .collect::<Vec<_>>()
         .join(&' ')
         .iter()
         .collect::<String>()
   


    
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars()
          .filter(|x| x.is_alphanumeric())
          .map(|x| encode_char(x))
          .collect()
   
}

fn encode_char(ch: char) -> char{
    match ch.is_ascii_digit(){
        false => ('a' as u8 * 2 + 25 - ch as u8) as char,
        true => ch
    }
}
