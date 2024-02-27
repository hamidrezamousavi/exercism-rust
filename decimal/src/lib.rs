use std::ops::Add;
use std::iter::zip;
const MAXLENGHT:usize = 5;
#[derive(Debug)]
enum Sign{
    Positive,
    Negative,
}
#[derive(Debug)]
pub struct Decimal {
    integral: [i8;MAXLENGHT],
    fractional:[i8;MAXLENGHT],

}

impl Decimal {
    pub fn new() -> Decimal{
        return Decimal{integral:[0; MAXLENGHT], fractional:[0; MAXLENGHT]};
    }
       
    pub fn try_from(input: &str) -> Option<Decimal> {
    let mut input = input.to_string();
    let sign = Self::get_remove_sign(&mut input)?;
    let mut int:&str="";
    let mut frac:&str = "";
    match input.split_once('.'){
        Some((i,f)) => { int = i; frac = f;},
        None => int = &input,
    }
    let mut integral = [0;MAXLENGHT];
    let mut fractional = [0;MAXLENGHT];
    for (index, ch) in int.chars().rev().enumerate(){
        integral[index] = ch.to_digit(10)? as i8 * sign;
    }
    for (index, ch) in frac.chars().enumerate(){
        fractional[index] = ch.to_digit(10)? as i8 * sign;
    }
    fractional.reverse();
    
    Some(Self{integral, fractional})
    }

    fn get_remove_sign(input:&mut String)->Option<i8>{
        match input.chars().nth(0).unwrap(){
            '+' => { input.remove(0);return  Some(1)},
            '-' => { input.remove(0);return Some(-1)},
            c if c.is_digit(10) => return Some(1) ,
            other => return None,
        }
    }
    fn correct_sign(&mut self ){
        let sign = self.find_sign();
        let mut temp = 0;
        for (i, digit) in self.fractional.iter_mut()
                    .chain(self.integral.iter_mut()).enumerate(){
        *digit = *digit + temp;
        temp  = 0;                
        if *digit * sign < 0 {
            *digit = *digit + 10 * sign;
            temp = sign * -1;
        }
                    }
    }
    fn correct_digit(&mut self ){
        let sign = self.find_sign();
        let mut temp = 0;
        for (i, digit) in self.fractional.iter_mut()
                    .chain(self.integral.iter_mut()).enumerate(){
        *digit = *digit + temp;
        temp  = 0;                
        if *digit * sign > 9 {
            *digit = *digit - 10 * sign ;
            temp = sign * 1;
        }
                    }
    }
    fn find_sign(&self)->i8{
        for digit in self.integral.iter().rev(){
            match *digit{
                d if d> 0 => return 1,
                d if d< 0 => return -1,
                _ => (),
            };
        }
        for digit in self.fractional.iter().rev(){
            match *digit{
                d if d> 0 => return 1,
                d if d< 0 => return -1,
                _ => (),
            };
        }
        return 1;
    }


}

impl Add for Decimal{
    type Output = Self;
    fn add(self, rhd:Self) -> Self{
        let mut result = Decimal::new();
        for (i,(s, r)) in zip(self.integral, rhd.integral).enumerate(){
            result.integral[i] = s + r;
        }
        for (i,(s, r)) in zip(self.fractional, rhd.fractional).enumerate(){
            result.fractional[i] = s + r;
        }
        result.correct_sign();
        result.correct_digit();
        return result;
    }
}
