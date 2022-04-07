// rust uses pythonic if statments where you don't need the brackets
pub fn show_ifs() {
    let n = 3;
    if n > 3 {
        println!("This number is bigger than three")
    } else if n == 3 {
        println!("This number is equal to three")
    } else {
        println!("This number is bigger than three")
    }
}