// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct

struct Color2(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        // Self references the struct itself, immutable by default.
        // We could use self instead of &self but that would borrow the variable
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        // Again, we're referencing the struct but with a mutable reference as
        // we're modifying it
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        // 
        (self.first_name, self.last_name)

    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c2 = Color2(255, 0, 0);
    c2.0 = 120;
    println!("Color: {} {} {}", c2.0, c2.1, c2.2);

    let mut p = Person::new("Mary", "Doe");
    println!("Person {}", p.full_name());
    p.set_last_name("Williams");
    // println!("Person {} {}", p.first_name, p.last_name);
    println!("Person {}", p.full_name());
    println!("Person Tuple {:?}", p.to_tuple());
}
