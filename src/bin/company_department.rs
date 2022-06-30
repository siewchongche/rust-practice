// Using a hash map and vectors, create a text interface to allow a user to 
// add employee names to a department in a company. For example, 
// “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user 
// retrieve a list of all people in a department or all people in the company 
// by department, sorted alphabetically.

use std::{io, collections::HashMap};

fn main() {
    let mut departments = HashMap::new();

    loop {
        println!("------------------------------------------");
        println!("Input 1 for insert new command");
        println!("Input 2 for retrieve people in department");
        println!("Input 3 for retrieve all people in company");
        println!("Input 0 for exit");
        println!("------------------------------------------");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input!");
                continue
            }
        };

        if input == 1 {
            println!("Input your command");
            let mut command = String::new();
            io::stdin()
                .read_line(&mut command)
                .expect("failed to read line");

            let command: Vec<&str> = command.split(" ").collect();
            let person = String::from(command[1]);
            let department = String::from(command[3].trim());
            let department_ = department.clone();
            let workers = departments.entry(department).or_insert(vec![]);
            workers.push(person);
            println!(
                "Success add {} to {} department",
                workers[workers.len() - 1],
                department_
            );
        } else if input == 2 {
            println!("{}", "Input the department to retrieve:");
            let mut department = String::new();
            io::stdin()
                .read_line(&mut department)
                .expect("failed to read line");
            let department = department.trim();
            let people = departments.get(department);
            match people {
                Some(worker_list) => {
                    let mut worker_list_vector = Vec::new();
                    for worker in worker_list {
                        worker_list_vector.push(worker);
                    }
                    worker_list_vector.sort();
                    println!("Department: {}", department);
                    println!("{}", "People:");
                    for worker in worker_list_vector {
                        println!("- {}", worker);
                    }
                },
                None => {
                    println!("{} department is not exist yet!", department);
                    continue
                }
            }
        } else if input == 3 {
            println!("All people in the company by department:");
            for (department, people_list) in departments.iter() {
                println!("{}", department);
                for people in people_list {
                    println!("- {}", people);
                }
            }
        } else {
            break;
        }
    }
}