use std::str::from_utf8;

fn set_len(ch:char)-> usize{
    ((ch as u8 - b'A')*2 + 1)as usize
    
}
pub fn get_diamond(ch: char) -> Vec<String> {
    let str_len = set_len(ch);
    let mut v = vec!(vec!(b' ' ;str_len);(ch as u8 - b'A' + 1).into());
    for (i, ch) in ('A'..=ch).rev().enumerate(){
        v[i][i] = ch as u8;
        v[i][str_len -i-1] = ch as u8;
    }
    let mut half:Vec<String> = Vec::new();
    for item in v{
        half.push(from_utf8(&item).unwrap().to_string());
    }
    let mut out = half.clone();
    out.reverse();
    out.pop();
    out.append(&mut half);
    out
    
}
