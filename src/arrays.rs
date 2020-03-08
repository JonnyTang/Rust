pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Re-assign value
    numbers[2] = 20;

    println!("{:?}", numbers);
    println!("Sigle Value: {}", numbers[0]);

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Array are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}
