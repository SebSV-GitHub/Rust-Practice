const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // Constants

fn main() {
    let mut x = 5;
    println!("{x}!");
    x = 6; // Variable mutation, wont work without mut
    println!("{x}!");
    println!("{THREE_HOURS_IN_SECONDS}!");

    // Shadowing
    let y = 4;
    let y = y + 1; // Old y still exist but the name y changed its location

    {
        let y = y * 2; // Here the y variable overshadows the old variable taking it name, this
                       // will end when the block scope ends
        println!("{y}")
    }
    println!("{y}");

    let y = "Hello, world!";

    println!(y); // Shadowing also lets you change the variable type
}
