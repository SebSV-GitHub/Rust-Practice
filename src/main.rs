// Rust uses ownership to manage memory
// That means that you can specifycally free memory
// Or Rust will do it when it comes out of scope
//
// Fixed size variables will be pushed to the memory stack
// The stack is a more efficient memory than the heap

fn main() {
    let x = 4;
    let y = x;
    println!("{x}");
    println!("{y}"); // This works because as a variable with a fixed size
                     // The implementation of the copy trait is not expensive
    let str_fixed = "Sample text"; // String literals behave the same, they
                                   // have a fixed size assigned in compile time
                                   //
    let str_fixed_2 = str_fixed;

    println!("{str_fixed} {str_fixed_2}"); // This allows the usage of both variables
                                           // Because the were added to the binary
                                           // at compile time

    let mut str = String::from("Hello"); // This call with String, however,
                                         // is not fixed and have 3 components
                                         // a pointer to the heap memory,
                                         // a length and a capacity
                                         // This implementation allow mutabilityo
    str.push_str(", world!"); // This allowcation happens at runtime, therefore
                              // The amount of memory is unkown and will be pushed
                              // to the memory heap
    println!("{str}");

    let str_2 = str; // However since the declaration of str is the heap,
                     // The ownership of the variable changes and it's
                     // "moved" to str_2; this makes str unusable because
                     // Rust only allows one owner at a time
    println!("{str_2}");
    //println!("{str}"); // This throws an error

    // A similar things happens to the functions
    let fn_str = String::from("Another test");
    let fn_str_2 = takes_and_gives_back(fn_str);
    let fn_str_3 = gives_ownership(); // some_string ownership is now borrowed by fn_str_3
                                      // println!("{fn_str}"); // This doesn't work, takes_and_gives_back borrowed the ownership
                                      // However, the function return back the ownership to fn_str_2
    println!("{fn_str_2} ");
    println!("{fn_str_3} ");
} // Here fn_str_2 and fn_str_3 are now out of scope, therefore, Rust frees the memory

fn gives_ownership() -> String {
    let some_string = String::from("Test");
    some_string
}

fn takes_and_gives_back(str: String) -> String {
    str
}
