pub fn is_leap_year(year: u64) -> bool {
    
    if year % 100 == 0 {
        if year % 4 == 0 && year % 400 == 0{
            return true;
        }else{
            return false;
        }
    }else{
        if year % 4 == 0 {
            return true;
        }else{
            return false;
        }

    }
    
   
}
