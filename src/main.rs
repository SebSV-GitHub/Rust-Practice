fn main() {
    // Scalar types
    // - Integers
    let _signed_integer: i32 = 0; // Default value for numbers in Rust
    let _unsigned_integer: u32 = 0; // Signied values uses the two's complement

    let _decimal = 123_567;
    let _hex = 0xfff;
    let _octal = 0o77;
    let _binary = 0b1111_0000;

    // - Floating-points
    let _float32: f32 = 1.9;
    let _float64: f64 = 39.4; // Default value for Floating-points values

    // Operations
    let _sum = 4 + 3;

    let _difference = 494.3 - 42.5;

    let _product = 3f64 * 39.4;

    let _quotient = 3.0 / 4.3;
    let _truncated = 12 / 5; // Just integer division result (2)

    let _remainder = 43 % 4;

    // Booleans
    let _t = true;
    let _f = false;

    // Char, to infeer this value it must be declared with single quotes
    let _c = 'z';
    let _z: char = 'ℤ';
    let _emoji = '👽';

    // Compound types
    // - Tuple
    let tup: (i32, char, f64, bool) = (1, 'a', 3.4, true);

    let a = tup.0;
    let (x, y, z, b) = tup; // Get inner values with destructuring
    println!("x,a: {x},{a}, y: {y}, z:{z}, b: {b}");

    let _empty_value = (); // This is called a unit and represents an empty value

    // - Array
    let _arr = [1, 2, 3, 4, 5];

    let _a: [f64; 3] = [4.3, 23.5, 3.4];
    let ar = [4; 8]; // This will initialize an array with 8 spaces filled with a 4

    let arrx = ar[3];
    println!("ar: {arrx}");
}
