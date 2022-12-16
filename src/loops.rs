pub fn run() {
    let mut count: u8 = 0;

    // Infinite Loop
    loop {
        count += 1;
        println!("Number: {count}");
        if count == 20 {
            break;
        }
    }

    count = 0;

    // While Loop (FizzBuzz)
    while count <= 100 {
        fizz_buzz(count);
        count += 1;
    }

    // For Range
    for x in 0..100 {
        fizz_buzz(x);
    }
}


fn fizz_buzz(count: u8) {
    if count % 15 == 0 {
        println!("fizzbuzz");
    } else if count % 3 == 0 {
        println!("fizz");
    } else if count % 5 == 0 {
        println!("buzz");
    } else {
        println!("{count}");
    }
}