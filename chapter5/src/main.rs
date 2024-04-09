struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct DebuggableStruct {
    active: bool,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "user {}", self.username)
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("brit"),
        email: String::from("brit@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        active: false,
        // This 'moves' user1 as user1 has some fields with types that do not
        // implement the Copy trait.
        ..user1 
    };

    println!("{user2}");
    

    let d1 = DebuggableStruct {
        active: true
    };

    println!("{:?}", d1);
    println!("{:#?}", d1);
    dbg!(d1);
}

