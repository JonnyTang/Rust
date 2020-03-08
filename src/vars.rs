pub fn run() {
    let name = "jonny";
    let mut age = 32;
    age = 33;

    println!("my name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("jonny", 32);
    println!("{} is {}", my_name, my_age);
}