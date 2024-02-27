use rna_transcription::{Rna, Dna};

fn main(){
    let input = "ACGTGGTCTTAA";
    let output = Dna::new(input).unwrap().into_rna();
  //  let expected = Rna::new("UGCACCAGAAUU").unwrap();
    println!("{:?}", output);
}