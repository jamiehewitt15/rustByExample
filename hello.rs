// This is the main function
fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");
    println!("Hello Everyone!");
    println!("I'm a Rustacean!");

    // Comments:
    let x = 5 +  90 + 5;
    println!("Is `x` 10 or 100? x = {}", x);

    // Formatted print:
    println!("{} days", 31i64);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{object} {verb} {subject}",
             object="the smelly cat",
             subject="the big brown fox",
             verb="runs away from");
}