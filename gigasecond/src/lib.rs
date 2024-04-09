use time::PrimitiveDateTime as DateTime;
use time::Duration;

pub fn after(start: DateTime) -> DateTime {
    let giga_sec = 1000000000;
    start + Duration::new(giga_sec,0)
    
}
