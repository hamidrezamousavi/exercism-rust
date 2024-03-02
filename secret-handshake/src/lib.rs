pub fn actions(n: u8) -> Vec<&'static str> {
    let mut result = Vec::new();
    let actions_list = ["wink", "double blink", 
                        "close your eyes", "jump"];
    for i in 0..4{
        if (n >> i & 1) == 1 {
            result.push(actions_list[i]);
        }
    }
    if (n >>4 & 1)== 1 { result.reverse();}
    return result;
}
