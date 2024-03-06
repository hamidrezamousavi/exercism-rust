use affine_cipher::{decode, encode};

fn main() {
    let phrase = "odpoz ub123 odpoz ub";
    let (a, b) = (25, 7);
    let output = decode(phrase, a, b).unwrap();
    println!("{output}");
}
