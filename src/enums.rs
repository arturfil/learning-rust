pub enum TestEnum {
    A,
    B(i32),
    C {x: i32, y: i32}
}

// let a: TestEnum = TestEnum::A;
// let b: TestEnum = TestEnum::B(23);
// let c: TestEnum = TestEnum::C{x:32, y:4};

// if let TestEnum::B(val) = b {println!("{}", val)}