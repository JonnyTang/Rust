// Tuples group together value of different types
// Max 12 elements
pub fn run() {
    let person: (&str, &str, i8) = ("jonny", "tang", 32);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}