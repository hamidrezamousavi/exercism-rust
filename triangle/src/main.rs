use triangle::Triangle;
fn main(){
    let input = [4, 4, 4];
    
    let output =  Triangle::build(input).unwrap();
    println!("{}",output.is_isosceles());
}