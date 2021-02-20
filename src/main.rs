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

    let mut s = "123";
    println!("s is {}", s);
    // expected `&str`, found `usize`
    // s = s.len();
}

fn main() {
    print();
    variable()
}
