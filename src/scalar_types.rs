// Here we are going to put the types

pub fn print_var() {

    // u => unsigned
    // u8, u16, u32, u64, u128
    let test:u8 = 10;
    
    // i => signed integer
    // signed integer
    let signed:i8 = -100;
    
    // floats
    let float:f32 = 1.0;
    println!("signed: {}, unsigned: {}, float: {}", test, signed, float);

    // char
    let letter = "c";
    let emoji = "\u{1F600}";
    
    println!("letter: {}, emoji: {}", letter, emoji);
}
