// With Copy trait, return T
fn find_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &element in list {
        if element > largest {
            largest = element;
        }
    }
    largest
}

// Without Copy trait, return &T
// fn find_largest<T: PartialOrd>(list: &Vec<T>) -> &T {
//     let mut largest = &list[0];
//     for element in list {
//         if element > &largest {
//             largest = element;
//         }
//     }
//     largest
// }

fn main() {
    let integers = vec![2, 6, 3, 8, 1, 9, 5];
    let largest_integer = find_largest(&integers);
    println!("Largest integer: {}", largest_integer);

    let chars = vec!['s', 'e', 'c', 't', 'v', 'y', 'n'];
    let largest_char = find_largest(&chars);
    println!("Largest char: {}", largest_char);
}
