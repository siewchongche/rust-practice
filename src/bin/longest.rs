fn find_longest<'a>(string_a: &'a str, string_b: &'a str) -> &'a str {
    if string_a.len() > string_b.len() {
        return string_a
    } else {
        return string_b
    }
}

fn main() {
    let string = "This is a string";
    let another_string = "This is a longer string";

    let longest_string = find_longest(&string, &another_string);
    println!("The longest string is: {}", longest_string);
}
