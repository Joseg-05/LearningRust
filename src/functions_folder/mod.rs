mod structs_folder;
use structs_folder::my_car_struct::Car;
use std::io;
use std::io::Write;
pub fn my_first_module_function() -> () {
    let car: Car =  Car { color: String::from("Green"), make: String::from("Toyota") };
    println!("The color of my car is: {}, and the make is: {}", car.color, car.make);
    let tommy = String::from("My Name is Tommy");
    println!("{}", tommy);
    println!("Hello, world!");
}

pub fn getting_user_details() -> () {

    let name_of_bot = "Rusty";
    println!("Hi, My name is {0}", name_of_bot.trim_end());
    //immutable  cannot be changed
    let age_of_bot = 5i32;
    println!("I am {0} years old", age_of_bot);
    println!("What is your name?");

    let mut name = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
    .read_line(&mut name)
    .expect("Failed to read line");
    print!("Hi {0}, \nnice to meet you {0}!!!! \n\n", name.trim_end());
}