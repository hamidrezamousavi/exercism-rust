use luhn_trait::Luhn;

fn main(){
    println!("{:?}", 
    String::from("046 454 286").valid_luhn());
}