//Trait is like an interface in C#
pub trait Body {
    fn new(first_name:String, last_name: String, email: String) -> Self;
}
//We create a struct
pub struct Person{
    pub first_name: String,
    pub last_name: String,
    pub email: String,

}
//Here we implement the trait and were we can add a function
impl Body for Person{
     fn new(first_name: String, last_name: String, email: String) -> Self {
        Self {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            email: email
        }
    }
}
