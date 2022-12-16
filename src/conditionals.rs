pub fn run() {
    let age: u8 = 22;
    let check_id = false;
    let knows_person_of_age = true;

    // If/Else
    if age >= 18 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry m8.");
    } else {
        println!("Bartender: Plz show ID m8");
    }

    // Shorthand IF (why no ternary operator)
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {is_of_age}");
}
