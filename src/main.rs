// use crate::enums::TestEnum;
// use crate::interfaces::BirdTrait;
// use crate::interfaces::Bird;

use std::collections::HashMap;

pub mod scalar_types;
pub mod compound_types;
pub mod tuples;
pub mod functions;
pub mod mutability;
pub mod slicing;
pub mod strings;
pub mod if_statements;
pub mod loops;
pub mod structs;
pub mod interfaces;
pub mod enums;

fn main() {
    // scalar_types::print_var();
    // compound_types::showcase_arrays();
    // tuples::show_tuples();
    // let res = functions::is_even(4);
    // println!( "{}", res );

    // mutability::show_mut()
    // let test = [1,2,3,4,5,6];
    // let vector_arr = vec![1,2,3,4,5];
    // println!("{:?}", &vector_arr[..4]);
    // slicing::showing_slice();
    // // slicing::slice_vector(&mut vector_arr, &vector_arr[0 .. 4]);
    // slicing::borrow_slice(test, &test[0 .. 4]);
    // strings::show_strings();

    // if_statements::show_ifs();
    // loops::showing_loops();

    // loops::showing_switch_statements(0);
    // structs::show_structs();

    // ---- Create a class/struct with interface ---- //
    // let parrot = Bird {
    //     bird_type: "Parrot".to_string(),
    //     color: "Green".to_string(),
    //     weight: 234
    // };

    // parrot.fly_distance(1444,234);

    // let a: TestEnum = TestEnum::A;
    // let b: TestEnum = TestEnum::B(23);
    // let c: TestEnum = TestEnum::C{x:32, y:4};

    // if let TestEnum::B(val) = b {println!("{}", val)}

    // let vec3:Vec<u64> = (1..101).collect();
    // print!("[");
    // for i in vec3 {
    //     print!("{}, ", i);
    //     if i % 10 == 0 {
    //         println!();
    //     } 
    // }
    // print!("]");
    
    let mut map = HashMap::new();
    map.insert(1, "Hi ");
    map.insert(2, "This ");
    map.insert(3, "Is ");
    map.insert(4, "A Message ");
    match map.get(&2) { Some(str) => println!("{}", str), _=> println!("Doesn't exist in map")}
    println!("{:?}", map);

    fn test(num:u16) -> Option<u16> {
        if num > 5 {
            None
        } else {
            Some(num * 5)
        }
    }

    let res:Option<u16> = test(3);
    println!("{}", res.unwrap());
}
