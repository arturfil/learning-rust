pub struct Bird {
    pub bird_type: String,
    pub color: String,
    pub weight: u16
}

pub trait BirdTrait {
    fn fly_distance(&self, speed:u16, time:u16) -> u16;
}

impl BirdTrait for Bird {
    fn fly_distance(&self, speed: u16, time: u16) -> u16 { 
        let distance = speed/time;
        println!("The distance this bird flew is {}", distance);
        return distance;
     }
}