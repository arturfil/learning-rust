pub fn showing_loops() {
    // for i in 950..1000 {
    //     // println!("{}", i);
    // }
    let mut j = 1;
    
    while j < 20 {
        if j % 3 == 0 {
            j += 1;
            continue;
        } else if j == 17 {
            break
        }
        println!("{}", j);
        j += 1;
    }

    
}