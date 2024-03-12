pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.len() < len  { return vec!()};
    let output = digits.chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|d| d.iter().collect::<String>())
        .collect::<Vec<_>>();
    output              
}
