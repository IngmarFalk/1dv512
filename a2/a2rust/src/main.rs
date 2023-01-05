use std::time::Duration;

pub mod shared;
mod tasks;

fn main() {
    loop {
        let input = get_input();
        match input {
            1 => tasks::task1::task1(),
            2 => tasks::task2::task2(),
            3 => tasks::task3::task3(),
            _ => unreachable!(),
        }
    }
}

fn get_input() -> u8 {
    let mut input = String::new();
    println!("Enter task number: \n1. Task 1\n2. Task 2\n3. Task 3\n");
    std::io::stdin().read_line(&mut input).unwrap();
    let res = input.trim().parse::<u8>();
    match res {
        Ok(res) => res,
        Err(_) => {
            println!("Invalid input");
            get_input()
        }
    }
}
