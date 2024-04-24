use rand::prelude::*;

static mut container:Vec<String> = vec!();

pub struct Robot{
    robot_name:String,    
}
impl Robot {
    pub fn new() -> Self {
        Self{robot_name:Self::make_unic_name()}
    }

    pub fn name(&self) -> &str {
        &self.robot_name
    }

    pub fn reset_name(&mut self) {
        self.robot_name = Self::make_unic_name();
    }

    fn make_rand_string()->String{
        let mut rng = thread_rng();
        let mut output = String::new();
        let alphabet = "ABCDEFGHIKLMNOPQRSTUVWYZ";
        let numbers = "1234567890";
        for i in 0..2{
            output.push(alphabet.chars().choose(&mut rng).unwrap());
        }
        for i in 0..3{
            output.push(numbers.chars().choose(&mut rng).unwrap());
        }
        output
    }

    fn make_unic_name()->String{
        let mut name= String::new();
        unsafe{
        while true{
            name = Self::make_rand_string();
            if !container.contains(&name){
                break;
            }
        }
        
        container.push(name.clone());
        }
        name

    }
}
