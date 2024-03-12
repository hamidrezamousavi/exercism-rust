use std::ops::{Add,Sub};
use std::cmp::{Eq, PartialOrd};

pub struct Triangle<T>
{
    a:T,
    b:T,
    c:T,
}

impl<T> Triangle<T> 
where 
T: Add<Output = T>+Sub<Output = T>+PartialOrd + Copy
{
    pub fn build(sides: [T;3] )-> Option<Triangle<T>> {
        let a = sides[0]; 
        let b = sides[1]; 
        let c = sides[2]; 
        if (a > b + c || b > a + c ||c > a + b) ||
            sides.iter().any(|x| *x == a - a)
        {
            return None;
        }  
        Some(Triangle{a,b,c})
    }

    pub fn is_equilateral(&self) -> bool {
        return self.a == self.b && self.b == self.c;
    }

    pub fn is_scalene(&self) -> bool {
        return self.a != self.b && self.b != self.c && self.a != self.c;
    }

    pub fn is_isosceles(&self) -> bool {
        return  !self.is_scalene(); 
    }
}
