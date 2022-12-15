pub fn run() {
    // Maximum of 12 elements in a tuple
    let person: (&str, &str, i8) = ("Brad", "Mass", 37);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
    
}
