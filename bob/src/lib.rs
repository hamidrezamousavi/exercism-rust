#[derive(Debug)]
enum Message{
    Question, 
    Yell, 
    YellQuestion, 
    Silence,
    Anything,
}

fn parse_message(message:&str)->Message{
    let message = message.replace(" ","");
    
    if message.len() == 0 {
        return Message::Silence;
    }
    if message.chars().all(|x| x.is_whitespace()){
        return Message::Silence;
    }
    
    
    if message.ends_with('?') &&
       message.chars().all(|x| x.is_uppercase()|| 
                           x.is_ascii_punctuation())&&
       message.chars().any(|x| x.is_alphabetic())                    
        {
        return Message::YellQuestion;
        }
    if message.ends_with('?') {
    return Message::Question;
    }
    if message.chars().all(|x| !x.is_alphabetic()){
        return Message::Anything;
    }
    if message.chars().all(|x| x.is_uppercase()|| !x.is_alphabetic()){
        return Message::Yell;
    }
    return Message::Anything;
}

pub fn reply(message: &str) -> &str {
    let answer = match parse_message(message){
                Message::Question =>"Sure.",
                Message::Yell => "Whoa, chill out!",
                Message::YellQuestion =>"Calm down, I know what I'm doing!", 
                Message::Silence =>"Fine. Be that way!",
                Message::Anything =>"Whatever.",
                   };
                  
 
    
    answer
}
