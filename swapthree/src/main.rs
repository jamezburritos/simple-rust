use std::ops::BitXor;

fn swapthree<T>(x: &mut T, y: &mut T, z: &mut T)
where
    T: BitXor<Output = T> + Copy 
{
   *x = *x ^ *y; 
   *y = *x ^ *y; 
   *z = *x ^ *y; 
   *x = *x ^ *y; 
}

fn main() {
    let mut x = 12;
    let mut y = 13;
    let mut z = 14;

    println!("Before swapping: {}, {}, {}", x, y, z);
    swapthree(&mut x, &mut y, &mut z);
    println!("After swapping: {}, {}, {}", x, y, z);
}
