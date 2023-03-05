// Functions

fn main() {
    println!("Hello, world!");
    another_function(3);
}

fn another_function(x: i32) {
    let y = five();
    let y = plus_one(y);
    println!("Other function invoked with x: {x}, y: {y}");
}

fn five() -> i32 {
    // This function uses the expression 5 (without the semicolon) to return that
    // number
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
