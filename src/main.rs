pub mod scalar_types;
pub mod compound_types;
pub mod tuples;
pub mod functions;
pub mod mutability;
pub mod slicing;

fn main() {
    // scalar_types::print_var();
    // compound_types::showcase_arrays();
    // tuples::show_tuples();
    // let res = functions::is_even(4);
    // println!( "{}", res );

    // mutability::show_mut()
    let test = [1,2,3,4,5,6];
    slicing::showing_slice();
    slicing::borrow_slice(test, &test[0 .. 4])
}
