fn print() {
    println!("Hello, world, i want fix more bug!");

    let a = 12;
    // Cannot assign twice to immutable variable
    // a = "122";
    println!("a is {}", a);

    println!("{{}}")
}

fn main() {
    print()
}
