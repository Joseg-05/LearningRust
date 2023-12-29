mod structs_folder;
use structs_folder::my_car_struct::Car;
pub fn my_first_module_function() -> () {
    let car: Car =  Car { color: String::from("Green"), make: String::from("Toyota") };
    println!("The color of my car is: {}, and the make is: {}", car.color, car.make);
    let tommy = String::from("My Name is Tommy");
    println!("{}", tommy);
    println!("Hello, world!");
}