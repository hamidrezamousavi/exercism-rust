use std::iter::Iterator;
use std::cmp::Ordering;
#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num <= 0 {return None;}

    let aliquots = Aliquot::new(num);
    match aliquots.sum::<u64>().cmp(&num){
        Ordering::Equal => return Some(Classification::Perfect),
        Ordering::Less => return Some(Classification::Deficient),
        Ordering::Greater => return Some(Classification::Abundant),
        _ => return None,
    }
  
}
struct Aliquot{
    num:u64,
    aliquot:u64,
}
impl Aliquot{
    fn new(num:u64)->Self{
        Self{num, aliquot:1}
    }
}
impl Iterator for Aliquot{
    type Item = u64;
    fn next(&mut self)->Option<u64>{
        
        for i in self.aliquot..=self.num/2{
            if self.num % i == 0{
                self.aliquot = i +1;
                return Some(i);
            }
        }
        None
    } 
}
