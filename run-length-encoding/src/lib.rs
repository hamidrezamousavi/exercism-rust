pub fn encode(source: &str) -> String {
   if source.len() == 0 {
       return "".to_string();
   } 
   let mut result = String::new();
   let mut counter = 0;
   let mut perv_ch = source.chars().nth(0).unwrap();
   for (i,ch) in source.chars().enumerate(){
       if ch == perv_ch{
           counter += 1;
       }else{
           if counter > 1 {
             result.push_str(&counter.to_string());
           }
           result.push(perv_ch);
           counter = 1;
           perv_ch = ch;
       }
       if i == source.len()-1{
        if counter > 1 {
            result.push_str(&counter.to_string());
          }
           result.push(perv_ch);
       }

   } 
   return result;
}

pub fn decode(source: &str) -> String {
    let result:String = source.split_inclusive(|c:char| c.is_alphabetic() || c.is_whitespace())
         .map(|s| convert(s)).collect();
    return result;
}
fn convert(s:&str)->String{
    if s.len() == 1 {
        return s.to_string();
    }
    let (count, st) = s.split_at(s.len()-1);
    let count:usize = count.parse::<usize>().unwrap();
     
    return st.repeat(count);
}
