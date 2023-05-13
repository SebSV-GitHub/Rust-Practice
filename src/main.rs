fn main() {
    let mut str = String::from("Hello, world!");
    let len = calculate_length(&str);
    println!("{}{}", str, len); // We can use str here because calculate_length
                                // is just using a reference, therefore, s is not
                                // droped when passed to the function
    change(&mut str); // This reference ends here, because it was used
    println!("{str}");

    // You can have all the reference you need
    // You cannot mix mutable and immutable references
    let _r1 = &str;
    let _r2 = &str;

    // However you cannot have more than one mutable reference
    // let _r1 = &mut str;
    // let _r2 = &mut str; // This will generate an error to prevent data races
    // if you need more than 1 mutable reference you need to create a new scope

    let _mut_r1 = &mut str;
    {
        let _mut_r2 = &mut str;
    }
}

fn calculate_length(s: &String) -> usize {
    // Using this refences the value of str,
    // s does not have ownership is just a reference
    // it also cannot be mutated
    s.len()
}

fn change(s: &mut String) {
    // Adding mut to the refence allows the funciton
    // to mutate the refence!

    s.push_str(" rustaceans")
}
