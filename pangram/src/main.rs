use pangram::is_pangram;

fn main(){
    let sentence = "Victor jagt zwölf Boxkämpfer quer über den großen Sylter Deich.";
    println!("{}", is_pangram(sentence));
}