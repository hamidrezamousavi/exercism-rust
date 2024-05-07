use roman_numerals::Roman;

fn main(){
    let input = 27;
    let output = Roman::from(input).to_string();
    println!("{output}"); 
}