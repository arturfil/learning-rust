pub fn showcase_arrays() {
    // array of u8 of length 3;
    let arr: [u8; 3] = [1,2,3];
    let second_arr: [u8; 5] = [100; 5]; 
    
    println!("index: {}, lenght: {}", arr[0], second_arr.len());
    println!("{:?}", second_arr);
}
