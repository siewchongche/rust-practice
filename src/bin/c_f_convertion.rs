use std::io;

fn main() {
    println!("{}", "Convert temperatures between Fahrenhelt and Celsius");

    let choose = loop {
    // loop {
        let mut index = String::new();

        println!("{}", "----------------------------------");
        println!("{}", "Choose 1 for Celsius -> Fahrenhelt");
        println!("{}", "Choose 2 for Fahrenhelt -> Celsius");
        println!("{}", "----------------------------------");

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let choose: String = index.trim()
            .parse()
            .expect("Parse failed");

        if choose == String::from("1") {
            break choose
        } else if choose == String::from("2") {
            break choose
        } else {
            println!("Not a wise choice");
        }
    };

    println!("{}", "Your input:");
    let degree = loop {
        let mut degree = String::new();
        io::stdin()
        .read_line(&mut degree)
        .expect("Failed to read line");
        let degree: f64 = match degree.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        break degree
    };

    if choose == String::from("1") {
        // Celsius -> Fahrenhelt
        let fahrenhelt = (degree * 9.0 / 5.0) + 32.0;
        println!(
            "{} Celsius = {} Fahrenhelt",
            (degree * 100.0).round() / 100.0,
            (fahrenhelt * 100.0).round() / 100.0
        );
    } else {
        // Fahrenhelt -> Celsius
        let celsius = (degree - 32.0) * 5.0 / 9.0;
        println!(
            "{} Fahrenhelt = {} Celsius",
            (degree * 100.0).round() / 100.0,
            (celsius * 100.0).round() / 100.0
        );
    }
}
