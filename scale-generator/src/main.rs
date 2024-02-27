use scale_generator::Scale;



fn main() {
    let s = Scale::new("G","mAMMMmm",).unwrap();
    println!("{:?}", s.enumerate());
     
}