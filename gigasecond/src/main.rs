use gigasecond::after;
use time::PrimitiveDateTime as DateTime;
use time::{Date, Time};
fn main(){
    
    let start = DateTime::new(
        Date::from_calendar_date(2011, 4.try_into().unwrap(), 25).unwrap(),
        Time::from_hms(0, 0, 0).unwrap(),
    );
    println!("{:?}",after(start));
}