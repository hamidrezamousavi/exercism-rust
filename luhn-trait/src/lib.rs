use std::string::ToString;
pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}
impl<T> Luhn for T
where T: ToString,
{
    fn valid_luhn(&self) -> bool {
        let data = self.to_string();
        let code = data.replace(" ","");
        if contain_non_digits(&code){
            return false;
        }
        if code.len() < 2 {
            return false;
        }
        
        let mut sum_of_all = 0;
        for (i, mut digit) in code.char_indices().rev(){
            if (code.len() - i) % 2 == 0{ 
                digit = double(digit);
            }
      
            sum_of_all += digit.to_digit(10).unwrap();
        }
    
    
        return sum_of_all % 10 == 0; 
    }
  

}
fn contain_non_digits(code:&str) ->bool{
    for ch in code.chars(){
        if !ch.is_digit(10){
            return true;
        }
    }
    return false;
}
fn double(digit:char)->char{
    let mut d:u32 = digit.to_digit(10).unwrap();
    d *= 2;
    if d > 9 { d -= 9;}
    return char::from_digit(d,10).unwrap();
}