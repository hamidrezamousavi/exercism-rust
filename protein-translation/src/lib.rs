use std::collections::HashMap;
#[derive(Debug)]
pub struct CodonsInfo<'a> {
    codons_name:HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.codons_name.get(codon).cloned()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut check = true;
        if rna.len() % 3 != 0{
            check = false;
        }
        let mut output:Vec<&'a str> = Vec::new();
        for mut i in 0..rna.len()/3{
            if self.codons_name.contains_key(&rna[i*3..i*3+3]){
                if self.codons_name[&rna[i*3..i*3+3]]== "stop codon"{
                    check = true;
                    break;
                }
                output.push(self.codons_name[&rna[i*3..i*3+3]]);
            }else{
                return None;
            }
            
            i+= 3;

        }
        if !check { return None;} 
       // println!("{:?}",output);  
        Some(output)   
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
  let mut c =HashMap::new();
  for (codon, name) in pairs{
      c.insert(codon, name);
  }
  CodonsInfo{ codons_name:c}  
}
