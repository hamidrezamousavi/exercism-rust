use std::collections::HashMap;
const NUCLEOTIDES:[char;4] =['A', 'C', 'G', 'T'];
pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let nucs_count = nucleotide_counts(dna)?;
    if !NUCLEOTIDES.contains(&nucleotide){return Err(nucleotide);}
    return Ok(nucs_count[&nucleotide]);
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut result = HashMap::new();

    for nuc in NUCLEOTIDES{
        result.insert(nuc,0);
    }

    for ch in dna.chars(){
        if !NUCLEOTIDES.contains(&ch){
            return Err(ch);
        }
        result.get_mut(&ch).map(|nuc| *nuc += 1).ok_or(ch)?;

    }
    return Ok(result);
}
