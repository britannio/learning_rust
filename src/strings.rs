pub fn run() {
    // String::from creates a growable string vs the fixed size &str type
    let mut hello = String::from("Hello");

    // Get length
    println!("Length: {}", hello.len());

    // Append char
    hello.push('W');

    // Append string
    hello.push_str("orld!");
    
    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());
    
    println!("Is Empty: {}", hello.is_empty());
    
    println!("Contains 'world' {}", hello.contains("World"));

    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{word}");
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    
    // Assertion testing (does this run in production?)
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{s}");

    // println!("{}", hello);
}
