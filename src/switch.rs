pub fn showing_switch_statements(mut k: u8) {
    while k < 100 {
        match k {
            k if k % 3 == 0 => println!("Fizz"),
            k if k % 5 == 0 => println!("Buzz"),
            k if k % 3 == 0 && k % 5 == 0 => println!("FizBuzz"),
            _ => println!("{}", k)
        }
        k += 1;
    }

}