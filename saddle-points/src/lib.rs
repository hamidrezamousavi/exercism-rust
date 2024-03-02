fn is_max_in_row(index:(usize,usize), input:&[Vec<u64>])->bool{
    return input[index.0][index.1] == *input[index.0].iter().max().unwrap()
}

fn is_min_in_col(index:(usize,usize), input:&[Vec<u64>])->bool{
    let row_number = input.len();
    let col = index.1;
    for row in 0..row_number{
        if input[index.0][index.1] > input[row][col]{
            return false;
        }
    }
    return true;
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for (row,rows) in input.iter().enumerate(){
        for (col, cols) in rows.iter().enumerate(){
            if is_max_in_row((row,col), input){
               if is_min_in_col((row,col),input){
                   result.push((row,col));
               }
            }
        }
    }
    return result;
}
