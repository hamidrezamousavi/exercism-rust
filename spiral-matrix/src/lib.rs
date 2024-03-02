pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut result = vec!(&[vec!(&[0;size]);size]); 
    let mut top = 0;
    let mut right = size ;
    let mut bottom = size;
    let mut left = 0;
    let mut index = (0,-1);
    while left< right && top < bottom{
        for _ in left..right{
            index.1 += 1;
            println!("{:?}", index);
        }
     //   println!("right move ended");
        top += 1;
        for _ in top..bottom{
            index.0 += 1;
            println!("{:?}",index);
        }
     //   println!("down move ended");
        right -= 1;
        for _ in left..right{
            index.1 -= 1;
            println!("{:?}",index);
        }
     //   println!("left move ended");
        bottom -= 1;
        for _ in top..bottom{
            index.0 -= 1;
            println!("{:?}",index);
        }
        left += 1;

        println!("{left}-{right}-{top}-{bottom}");
    }
    
    return vec!(vec!(1,2,3),vec!(4,5,6));
}
