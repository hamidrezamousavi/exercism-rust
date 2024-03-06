use crypto_square::encrypt;

fn main(){
    let input = "If man was meant to stay on the ground, god would have given us roots.";
    println!("{}", encrypt(input));
}