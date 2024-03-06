const one_digit:[&str;10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const teen_digit:[&str;10] = ["ten","eleven","twelve","thriteen","fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
const two_digit:[&str;8] = ["twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
const order:[&str;7] = ["","thousand", "million","billion","trillion","quadrillion","quintillion"];
fn encode_under_thousand(n:u64)->String{
    let mut output = String::new();
    match n {
        0..=9 => output = format!("{}",one_digit[n as usize]),
        10..=19 => output = format!("{}", teen_digit[(n - 10) as usize]),
        20..=99 =>{
            if n % 10 == 0 {
                output = format!("{}", two_digit[((n / 10)-2) as usize]);
            }else{
                output = format!("{}-{}", two_digit[((n / 10)-2) as usize],encode_under_thousand(n % 10));
            }
        },
        100..=999 =>{
            if n % 100 == 0 {
                output = format!("{} hundred", one_digit[(n / 100) as usize]);
            }else{
                output = format!("{} hundred {}", one_digit[(n / 100) as usize],encode_under_thousand(n % 100));
            }
        },
        _ =>() ,
    };
    return output;
}

pub fn encode(n: u64) -> String {
    let mut temp:u64 = n;
    let mut output = String::new();
    let mut i:usize = 0;
    if n == 0 {
        return "zero".to_string();
    }
    while temp > 0{
        if temp % 1000 == 0  { temp = temp /1000; i += 1; continue;}
        output = format!("{} {} {}",
                encode_under_thousand(temp % 1000),
                order[i],    
                output);
        temp = temp / 1000;
        i += 1;
    }
    output = (&output).trim().to_string();
    return output;    
}
