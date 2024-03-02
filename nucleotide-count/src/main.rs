use nucleotide_count::{count,nucleotide_counts}; 
fn main(){
    let nucleotide = 'X';
    let dna = "G";
    println!("{:?}",count(nucleotide,dna));
    println!("{:?}",nucleotide_counts(dna));
    

}