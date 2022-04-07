pub fn show_strings() {
    let string:&str = "hello world";
    let my_name: String = String::from("Arturo");
    println!("{}", string);
    println!("{}",my_name);

    // let sliced_str = &my_name[..8];

    let mut player: &str = "Crisitano Ronaldo";
    println!("{}", player);
    player = "Messi";
    println!("{}", player);
}