fn is_prime(n:u32)->bool{
    if n == 1 || n == 0 {return false;}
    for i in 2..=n/2{
        if n % i ==0 {
            return false
        }
    }
    true
}
pub fn nth(n: u32) -> u32 {
    let numbers = 0..;
    numbers.filter(|x| is_prime(*x))
           .nth(n as usize)
           .unwrap()
}
