/// Study log, for memory
///
/// # Examples
/// markdown rule, must be ///, very interesting
/// ```
/// type_float();
/// math();
///
/// ```
///

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

fn type_float() {
    // 64bit float
    let float64 = 2.9;
    // same as
    // let float64: f64 = 2.0;
    println!("float64 is: {}", float64);

    let float32: f32 = 3.2;
    println!("float32 is: {}", float32);
}

fn type_other() {
    // boolean
    let boolean: bool = true;
    println!("result is: {0}", boolean);

    // char
    let char1 = 'z';
    let char2: char = 'ℤ';
    let heart_eyed_cat: char = '😻';
    println!("result is: {0}, {1}, {2}", char1, char2, heart_eyed_cat);

    // tuple
    let tup = (500, 6.4, 1, "str");
    println!("result is: {}", tup.3);
    // error:  expected a tuple with 4 elements, found one with 2 elements
    // let (x, y) = tup;
    let (x, y, z, w) = tup;
    println!("x is: {}, y is: {}, z is: {}, w is : {} ", x, y, z, w);

    // array
    let array: [&str; 3] = ["Jan", "Feb", "March"];
    println!("array 0 : {}", array[0]);
    // var: [type, size] = [v0, v1];
    let int_array: [i32; 2] = [432, 356];
    println!("index 1 value: {}", int_array[1]);
    // reassign
    let [i1, i2] = int_array;
    println!("i1 value: {}, i2 value: {}", i1, i2);
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

/// add two unsigned integer variable
/// # Example
/// ```
/// let sum = (5, 10);
/// ```

// error: Function `add` cannot have anonymous parameters
// fn add(v1, v2) -> u32 {
fn add(v1: u32, v2: u32) -> u32 {
    // block can tip strong association logic
    let result = {
        v1 + v2
    };
    return result;
}

fn math() {
    let sum = add(5, 10);
    // unsupported ++ or --
    // let sum = 5 ++;
    let difference = 95.6 - 4.3;


    /// multiply 2 integer variable
    /// # Example
    /// ```
    ///
    /// ```
    fn multi(v1: i32, v2: i32) -> i32 {
        v1 + v2
    }
    let product = multi(4, 30);
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("result is: {0}, {1}, {2}, {3}, {4}", sum, difference, product, quotient,
             remainder);
}


fn condition() {
    fn how_old(age: u8) -> &'static str {
        let result;

        if age < 18 {
            result = "children";
        } else if age >= 18 && age < 30 {
            result = "teenager";
        } else {
            // same as: a ? b : c
            result = if age >= 100 { "not a person" } else { "master" };
        }

        return result;
    }

    let res1 = how_old(6);
    println!("res1 is: {}", res1);
    let res2 = how_old(18);
    println!("res2 is: {}", res2);
    let res3 = how_old(31);
    println!("res3 is: {}", res3);
    let res4 = how_old(100);
    println!("res4 is: {}", res4);
}

fn main() {
    print();
    variable();
    type_integer();
    type_float();
    type_other();
    base();
    math();
    condition()
}
