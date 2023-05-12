fn main() {
    let number = 3;
    if number < 5 {
        println!("True")
    } else {
        println!("Hello, world!");
    }

    //If as an statement
    let condition = true;
    let value = if condition { 7 } else { 3 };
    println!("{value}");

    loop {
        // Infinite loop
        break;
    }

    // You can use loops to return values, in cases where failing is an option

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };
    println!("{result}");

    // You can use loop labels to stop nested loops
    let result = 'one: loop {
        println!("Loop 1");
        loop {
            println!("Loop 2");
            break 'one 1;
        }
    };

    println!("{result}");

    // While loops
    let mut number = 10;
    while number >= 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!");

    // For loop
    let a = [10, 20, 30];
    for element in a {
        println!("{element}")
    }
}
