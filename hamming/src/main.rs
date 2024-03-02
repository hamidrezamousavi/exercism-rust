use hamming::hamming_distance;
fn main(){
    let first = "ACCAGGG";
    let second =  "ACTATGG";
    println!("{:?}", hamming_distance(first, second));
}