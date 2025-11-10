fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over",
    );

    println!("Base 10: {}", 69420);
    println!("Base 2 (binary): {:b}", 69420);
    println!("Base 8 (octal): {:o}", 69420);
    println!("Base 16 (hexadecimal): {:x}", 69420);

    // Four white spaces and a "1", for total width of 5
    println!("{number:>5}", number = 1);

    // Pad numbers with extras zeroes
    println!("{number:0>5}", number = 1);

    println!("{number:0<5}", number = 1);
    println!("{number:0>width$}", number = 1, width = 5);

    #[allow(dead_code)]
    struct Structure(i32);

    let number: f64 = 1.0;
    let width: usize = 5;

    println!("{number:>width$}");
}
