pub fn showing_slice() {
    let arr = [0, 1, 2, 3];
    let slice = &arr[1 ..3]; // [1, 2]; the .. are not inclusive
    println!("{:?}", slice);
}

pub fn borrow_slice(arr: [u8; 6], slice: &[u8]) {
    println!("{:?}", arr);
    println!("{:?}", slice);
    println!("length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);
}