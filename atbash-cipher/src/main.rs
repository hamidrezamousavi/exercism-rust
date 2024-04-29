use atbash_cipher::encode;
fn main(){
    let input = "Testing,1 2 3, testing.";
    println!("{:?}", encode(input));
}