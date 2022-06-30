use std::io;

fn main() {
    println!("{}", "Generate the nth Fibonacci number");

    let nth = loop {
        let mut nth = String::new();
        io::stdin()
        .read_line(&mut nth)
        .expect("Failed to read line");
        let nth: u32 = match nth.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        break nth
    };

    let mut answer = 0;
    if nth == 0_u32 {
        answer = 0;
    } else if nth == 1_u32 || nth == 2_u32 {
        answer = 1;
    } else {
        let mut minus_1 = 1;
        let mut minus_2 = 1;
        for _ in 0..nth - 2 {
            answer = minus_1 + minus_2;
            minus_2 = minus_1;
            minus_1 = answer;
        }
    }

    println!("The {}th Fibonacci number is {}", nth, answer);
}
