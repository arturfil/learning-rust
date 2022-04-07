pub struct Car {
    pub name: String,
    pub speed: u16,
}

pub fn show_structs() {
    let car = Car {
        name: "Ferrari".to_string(),
        speed: 100,
    };
    car.race();
}

impl Car {
    fn race(&self) {
        println!("{}", self.speed);
    }
}
