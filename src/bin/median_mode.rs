// Given a list of integers, use a vector and return the median 
// (when sorted, the value in the middle position) and 
// mode (the value that occurs most often; a hash map will be helpful here) 
// of the list.

use std::collections::HashMap;

fn main() {
    let list_of_integers = [5, 7, 9, 2, 8, 6, 9];
    // 2, 5, 6, 7, 8, 9, 9
    // median = 7
    // mode = 9

    let mut sorted_list: Vec<i32> = Vec::new();
    let mut temp_list = Vec::from(list_of_integers);
    while temp_list.len() > 0 {
        let mut lowest = temp_list[0];
        for element in &temp_list[1..] {
            if element < &lowest {
                lowest = *element;
            }
        }
        sorted_list.push(lowest);
        
        let mut found = false;
        let iterate_list = temp_list;
        temp_list = Vec::from([]);
        for element in iterate_list {
            if element == lowest {
                if found == false {
                    found = true;
                    continue;
                }
            }
            temp_list.push(element);
        }
    }
    // println!("{:?}", sorted_list);

    // let mut sorted_list = list_of_integers;
    // sorted_list.sort();
    let median_index = sorted_list.len() / 2;
    println!("Median of {:?} is {}", list_of_integers, sorted_list[median_index]);

    let mut map = HashMap::new();
    for element in list_of_integers {
        let count = map.entry(element).or_insert(0);
        *count += 1;
    }
    let mut mode = list_of_integers[0];
    for key in list_of_integers {
        let count = map[&key];
        if count > map[&mode] {
            mode = key;
        }
    }
    println!("Mode of {:?} is {}", list_of_integers, mode);
}