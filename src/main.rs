fn print() {
    println!("Hello, world, i want fix more bug!");

    let a = 12;
    // Cannot assign twice to immutable variable
    // a = "122";
    println!("a is {}", a);

    println!("{{}}")
}

fn variable() {
    // mutable variable
    let mut a = 123;
    println!("old a is {}", a);
    a = 456;
    println!("new a is {}", a);

    // assign a variable twice, variable can rebind
    let b = 123;
    println!("old b is {}", b);
    let b = 456;
    println!("b is {}", b);

    // const
    const C: i32 = 123;
    println!("C is {}", C);
    // cannot be named the same as a constant
    // let C = 456;

    // integer of 64bit
    let d: i64 = 64;
    println!("old d is {}", d);
    // unsigned integer
    let d: u64 = 128;
    println!("new d is {}", d);
    // Shadowing
    // new d is a Shadow of old d.

    let s = "123";
    // let mut s = "123";
    println!("s is {}", s);
    // expected `&str`, found `usize`
    // s = s.len();
}

fn type_integer() {
    // 8 bit signed int
    let int8: i8 = -8;
    println!("int8 is: {}", int8);

    // unsigned int, unsigned values cannot be negated
    // let un_int8: u8 = -8;
    let un_int8: u8 = 8;
    println!("un_int8 is: {}", un_int8);

    let int16: i16 = -16;
    println!("int16 is: {}", int16);

    let un_int16: u16 = 16;
    println!("un_int16 is: {}", un_int16);

    // TODO i32
    // TODO u32

    // TODO i64
    // TODO u64

    // TODO i128
    // TODO u128

    //  signed int of arch platform , 32bit or 64bit
    let int_size: isize = -16;
    println!("int_size is: {}", int_size);
}

fn base() {

    // 98,150
    let decimal = 98_150;
    println!("decimal is: {}", decimal);

    let hex = 0xf;
    println!("hex is: {}", hex);

    let octal = 0o17;
    println!("octal is: {}", octal);

    let binary = 0b1111;
    println!("binary is: {}", binary);

    // TODO ??
    // let byte = b"";
}

fn main() {
    print();
    variable();
    type_integer();
    base()
}
