// Variables hold primitive data or references to data
// Variables are immutable by default
// rust is a block-scoped language


pub fn run() {
  let name = "Andy";
  let mut age = 39;
  println!("My name is {} and I am {}.", name, age);

  age = 40;
  println!("My name is {} and I am {}.", name, age);

  // Define constant
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assign multiple variables
  let ( my_name, my_age ) = ("Andy", 39);
  println!("{} is {}", my_name, my_age);

}
