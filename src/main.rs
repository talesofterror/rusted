use rand::prelude::*;

fn main() {
    println!("Hello, world!\n");

    // uint_randoms();

    // float_randoms();

    let mut mutable_string: &str = "Hi.";
    println!("This is a mutable string test: {}\n memory address: {:p}", mutable_string, &mutable_string);

    /* 
    In Rust all types must be of a known size. 

    String is left unsized because the compiler can't know how 
    big it is before it's compiled. 

    So when delcaring a variable of type str you reference the
    type with `&` instead of declaring the type outright. 
     */

    mutable_string = "Hello.";
    println!("This is a mutable string test: {}\n memory address: {:p}", mutable_string, &mutable_string);

    /*
    Coule not print with pointer as below. Still don't know what pointers are.
    println!("This is a pointer to mutable string test: {}", *mutable_test);
     */

    let mut mutable_number: u64 = 5;
    println!("This is a mutable unsigned 64-bit integer: {}\n memory address: {:p}", mutable_number, &mutable_number);

}

pub fn uint_randoms() {
    let x: u8 = random();
    let y: u16 = random();
    let z: u32 = random();
    let a: u64 = random();

    println!("u8 random = {}", x);
    println!("u16 random = {}", y);
    println!("u32 random = {}", z);
    println!("u64 random = {}", a);
}

fn float_randoms() {
    let mut rng = thread_rng();
    let x: f64 = rng.gen();
    let y = rng.gen_range(-10.0..10.0);
    println!("\nf64 random (thread_rng): {}", x);
    println!("unspecified type random (thread_rng): {}", y);
}
