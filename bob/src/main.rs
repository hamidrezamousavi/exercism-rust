use bob::reply;
fn main(){
    let message = "WHAT'S GOING ON?";
    let b = reply(message);
    println!("{}",b);
}