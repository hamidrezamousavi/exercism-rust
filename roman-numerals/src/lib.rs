use std::fmt::{Display, Formatter, Result};

pub struct Roman{
    d:String,
}

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        write!(_f, "{}", self.d)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let mut roman_num = String::new();
        roman_num.push_str(&Roman::to_roman(Roman::thousand_of(num)));
        roman_num.push_str(&Roman::to_roman(Roman::hundred_of(num)));
        roman_num.push_str(&Roman::to_roman(Roman::tens_of(num)));
        roman_num.push_str(&Roman::to_roman(Roman::ones_of(num)));
        Self{d:roman_num}
    }
    
}
impl Roman{
    fn thousand_of(n:u32)->u32{
        (n / 1000) * 1000
    }
    fn hundred_of(n:u32)->u32{
        ((n - Roman::thousand_of(n))/100) * 100
    }
    fn tens_of(n:u32) -> u32{
        (n - Roman::thousand_of(n)-Roman::hundred_of(n))/ 10 * 10
    }
    fn ones_of(n:u32) -> u32{
        n - Roman::thousand_of(n)-Roman::hundred_of(n)-Roman::tens_of(n)  
    }

    fn to_roman(n:u32)->String{
        match n {
            1 => "I".to_string(),
            2 => "II".to_string(),
            3 => "III".to_string(),
            4 => "IV".to_string(),
            5 => "V".to_string(),
            6 => "VI".to_string(),
            7 => "VII".to_string(),
            8 => "VIII".to_string(),
            9 => "IX".to_string(),
            10 => "X".to_string(),
            20 => "XX".to_string(),
            30 => "XXX".to_string(),
            40 => "XL".to_string(),
            50 => "L".to_string(),
            60 => "LX".to_string(),
            70 => "LXX".to_string(),
            80 => "LXXX".to_string(),
            90 => "XC".to_string(),
            100 => "C".to_string(),
            200 => "CC".to_string(),
            300 => "CCC".to_string(),
            400 => "CD".to_string(),
            500 => "D".to_string(),
            600 => "DC".to_string(),
            700 => "DCC".to_string(),
            800 => "DCCC".to_string(),
            900 => "CM".to_string(),
            1000 => "M".to_string(),
            2000 => "MM".to_string(),
            3000 => "MMM".to_string(),
            _ => "".to_string(),
        }
    }
}
