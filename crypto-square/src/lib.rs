fn find_col_size(len:usize)->usize{
    let mut col = (len as f32).sqrt()as usize;
    if col.pow(2) < len {
        col += 1;
    }
    return col ;
}
pub fn encrypt(input: &str) -> String {
    if input.len() == 0 {
        return "".to_string();
    }
    let normalized = input.chars()
         .filter(|c| c.is_alphanumeric())
         .map(|c| c.to_lowercase().to_string())
         .collect::<Vec<String>>();
         
    let  rect = normalized.chunks(find_col_size(normalized.len()))
                             .collect::<Vec<_>>(); 
    let mut output =String::new();
    for i in 0..rect[0].len(){
        for j in 0..rect.len(){
            if i < rect[j].len(){
                output.push_str(&rect[j][i]);
            }else{
                output.push(' ');
            }
        }
        output.push(' ');
    } 
    output.pop(); 
  //  println!("{:?}",output);
    output
}
