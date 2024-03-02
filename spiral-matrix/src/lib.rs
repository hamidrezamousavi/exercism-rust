pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut result = vec![vec![0;size as usize]; size as usize]; 
    let mut top = 0;
    let mut right = size ;
    let mut bottom = size;
    let mut left = 0;
    let mut index = (0,-1);
    let mut num = 0;
    while left< right && top < bottom{
        for _ in left..right{
            index.1 += 1;
            num += 1;
            result[index.0 as usize][index.1 as usize] = num;
        }
    
        top += 1;
        for _ in top..bottom{
            index.0 += 1;
            num += 1;
            result[index.0 as usize][index.1 as usize] = num;
        }
        right -= 1;
        for _ in left..right{
            index.1 -= 1;
            num += 1;
            result[index.0 as usize][index.1 as usize] = num;
        }
        bottom -= 1;
        for _ in top..bottom{
            index.0 -= 1;
            num += 1;
            result[index.0 as usize][index.1 as usize] = num;
        }
        left += 1;
 }
    
    return result ;
}
