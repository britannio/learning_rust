pub fn run() {
    // Type inferred as i32 by default
    let x = 1;

    // Inferred as f64
    let y = 2.5;

    let z: i64 = 463722837;

    // Find max size
    println!("min i8: {}", std::i8::MIN);
    println!("max i8: {}", std::i8::MAX);
    println!("max i32: {}", std::i32::MAX);
    println!("max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;

    // Get bool from expression
    let is_greater: bool = 10 < 5;

    // char
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
    println!("{}", 110.0 * 0.3)
}
