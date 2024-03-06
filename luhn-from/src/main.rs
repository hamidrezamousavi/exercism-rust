use luhn_from::Luhn;
fn main(){
    let l = Luhn::from("046 454 287");
    println!("{:?}", l.is_valid());
}