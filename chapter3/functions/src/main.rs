fn main() {
    println!("Hello, world!");

    another_function();
    let f4 = fibonacci(4);
    println!("fib 4 = {f4}");
    

    let s = String::from("this is the string s");
    takes_ownership(&s);
    
    // we can't use it again because takes_ownership destroys it...
    println!("{s}");
    // Unless takes_ownership accepts &String i.e. a string reference.
    println!("{}", dangle2());
}

fn another_function() {
    println!("Another function.");
}

fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}


fn takes_ownership(strin: &String) {
    println!("{strin}");
}

fn dangle2() -> String {
    let s = String::from("hello");
    s
}