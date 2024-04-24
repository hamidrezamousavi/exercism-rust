use rand;
use robot_name::Robot;
fn main(){
    let b = Robot::new();
    println!("{:?}",b.name());
}