fn num_to_array(num:u32)->Vec<u8>{
    let mut output = Vec::<u8>::new();
    let mut num = num;
    if num == 0 {
        return vec![0];
    }
    while num > 0{
        output.push((num % 10) as u8);
        num = num / 10;
    }

    return output;
}

pub fn is_armstrong_number(num: u32) -> bool {
    let num_array = num_to_array(num);
    let mut sum = 0u64;
    for n in &num_array{
        sum += (*n as u64).pow(num_array.len()as u32);
        
    }
    sum == num.into()
}
