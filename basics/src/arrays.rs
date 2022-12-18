use std::mem;

pub fn run() {
    // Fixed length (like Java)

    // Array of 5 signed 32 bit integers 
    let mut numbers: [i32; 5] = [1, 2, 3, 4, -5];
    
    // Re-assign value
    numbers[2] = 20;
    
    println!("{:?}", numbers);

    // Get single value
    println!("Single value: {}", numbers[0]);
    
    // Get length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));
    
    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}