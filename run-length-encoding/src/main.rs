use run_length_encoding as rle;

fn main(){
    let input = "zzz ZZ  zZ";
    let i = "3z 2Z2 zZ";
   // let coded = rle::encode(input);
    let uncode = rle::decode(&i);
    /*
    let output = rle::decode(&rle::encode(input));
    let expected = "zzz ZZ  zZ";
    assert_eq!(output, expected);
      */ 
    
    println!("{}",uncode);
}