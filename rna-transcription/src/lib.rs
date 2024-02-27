const DNANUC:[char;4] = ['A', 'C', 'G', 'T'];
const RNANUC:[char;4] = ['A', 'C', 'G', 'U'];

#[derive(Debug, PartialEq, Eq)]
pub struct Dna{
    strand: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna{
    strand:String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        match dna.chars().position(|x| !DNANUC.contains(&x)){
            Some(i) => return Err(i),
            None => return Ok(Self{strand:String::from(dna)}),
        }
    }

    pub fn into_rna(self) -> Rna {
        let mut ran_strand = self.strand.clone();
        ran_strand = ran_strand.replace("A", "U")
                               .replace("T", "A")
                               .replace("C", "K")
                               .replace("G", "C")
                               .replace("K", "G");
        Rna::new(&ran_strand).unwrap()
        

    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        match rna.chars().position(|x| !RNANUC.contains(&x)){
            Some(i) => return Err(i),
            None => return Ok(Self{strand:String::from(rna)}),
        }
      
        
    }
}
