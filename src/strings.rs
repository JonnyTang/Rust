/*
Primitive str = Immutable fixed-length string somewhere in memory
String = Growable, heap-allocated data structure - Use when you need to modify or won string data
*/
pub fn run() {
    let mut hello = String::from("Hello");

    // Get length
    println!("Length: {}", hello.len());

    // Push char
    hello.push('W');

    // Push string
    hello.push_str(" jonny");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check is empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("jonny"));

    // Replace
    println!("Replace: {}", hello.replace("jonny", "there"));

    // loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());

    println!("{}", s);
}