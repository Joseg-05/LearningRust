use crate::functions_folder::structs_folder::my_car_struct::Car;
use crate::functions_folder::enums_folder::my_weather_enum::Weather;
use crate::functions_folder::structs_folder::my_person_struct::Person;
use super::structs_folder::my_person_struct::Body;
use std::io;
use std::io::Write;
/*
    Return an empty tuple
 */
pub fn my_first_module_function() -> () {
    let car: Car =  Car { color: String::from("Green"), make: String::from("Toyota") };
    println!("The color of my car is: {}, and the make is: {}", car.color, car.make);
    let tommy = String::from("My Name is Tommy");
    println!("{}", tommy);
    println!("Hello, world!");
}
/*
    Return an empty tuple
*/
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
/*
    Return an empty tuple
*/
pub fn using_enums() -> () {
    let rainy: Weather = Weather::Rainy;

    math_base_given_enum(rainy);
}
/*
    Returns an empty tuple.*
    @param {Weahter} given, The Weather that is currently at
*/
pub fn math_base_given_enum(given: Weather) -> () {
    match given {
        Weather::Rainy => print!("Today weather is rainy \n\n"),
        Weather::Sunny => print!("Today weather is sunny \n\n"),
        Weather::Cloudy => print!("Cannot tell what is the weather \n\n")
    }
}
/*
    Return an empty tuple
*/
pub fn using_struct_with_traits_and_functions() -> () {
    //Creating a new instance of Type Person Struct
    let user_created: Person = Person::new("Tommy".to_string(), "Lay".to_string(), "tommyLay@gmail.com".to_string());
    //Destructure a struct
    let Person { first_name: x, last_name: y,email: z  } = user_created;
    //OutPut
    println!("The person that we just created using the beautiful Rust is:");
    println!("First name is: {} \n Last name is: {} \n The user email is: {}", x.trim_end(), y.trim_end(), z.trim_end());
}