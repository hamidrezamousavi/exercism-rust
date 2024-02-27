use decimal::Decimal;
fn main(){
let d = Decimal::try_from("-13.956").unwrap();
let b = Decimal::try_from("-98.07").unwrap();
//println!("{:?}",d);

println!("{:?}", d + b);
}