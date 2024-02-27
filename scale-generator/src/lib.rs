const SHARP:&[&str] = &["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];
const FLAT:&[&str] = &["A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab"];
const USING_FLAT:&[&str] = &["F", "Bb", "Eb", "Ab", "Db", "Gb", "d", "g", "c", "f", "bb", "eb"];

fn produce_chromatic(tonic:&str)->Vec<String>{
    let scale:&[&str];
    let mut tonic = String::from(tonic);
    if USING_FLAT.contains(&tonic.as_str()){
        scale = FLAT;
    }else{
        scale = SHARP;
    }
    let mut c = tonic.chars();
    tonic =   match c.next() {
          None => String::new(),
          Some(f) => f.to_uppercase().chain(c).collect(),
      };
    let i = scale.iter().position(|&x| x == tonic).unwrap();
    let mut left:Vec<String> = scale[i..].iter()
                                  .map(|x| x.to_string())
                                  .collect();
    let mut right:Vec<String> =  scale[0..=i].iter()
                         .map(|x| x.to_string())
                         .collect();
   left.append(&mut right);
   return left;
}
fn intervals_to_numbers(intervals:&str)->Vec<usize>{
    let mut temp:Vec<usize> = Vec::new();
    for item in intervals.chars(){
        match item {
            'm' => temp.push(1),
            'M' => temp.push(2),
            'A' => temp.push(3),
            _ => return vec!(),
        }
    }
    let mut result:Vec<usize> = Vec::new();
    result.push(0);
    let mut t:usize = 0;
    for item in temp{
        t += item;
        result.push(t);
    }
    return result;
}

#[derive(Debug)]
pub struct Error;

pub struct Scale{
   pub scale:Vec<String>
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let  chromatic_scale = produce_chromatic(tonic);
        let intervals =  intervals_to_numbers(intervals);
        let mut scale = Vec::<String>::new();
        for item in intervals{
            scale.push(chromatic_scale[item].clone());
        }
        
        Ok(Self{scale})
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        let scale = produce_chromatic(tonic);
        Ok(Self{scale})
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.scale.clone()
    }
}
