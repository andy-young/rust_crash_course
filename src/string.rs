/*
Primitive str = Immutable fixed-length sting somewhere in memory
String = Growable, heap-allocated data structure - Used when ..
.. you need to modify or own string data.

*/

pub fn run() {
  let hello = "Hello"; // Immutable fixed-length etc..
  println!("{}", hello);

  let mut hello_again = String::from("Hello again ");
  println!("{}", hello_again);

  // Get Length
  println!("Length of {} is {}.", hello, hello.len());
  println!("Length of {} is {}", hello_again, hello_again.len());

  // Push is for Char type..
  hello_again.push('A');

  // Use push_str() to concatenate more than 1 character.
  hello_again.push_str("ndy");
  println!("{}", hello_again);

  // Get capacity (num of bytes)..
  println!("Capacity: {}", hello_again.capacity());

  // Check for empty string..
  println!("Is Empty: {}", hello_again.is_empty());

  // find substring
  println!("Contains 'Andy' {}", hello_again.contains("Andy"));

  // Replace
  println!("Replace: {}", hello_again.replace("Andy", "Vony"));

  // Loop through strings..
  for word in hello_again.split_whitespace() {
    println!("{}", word);

  // Create a string with defined capacity
  let mut s = String::with_capacity(10);
  s.push('>');
  s.push('<');

  // Assertion Testing
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity()); // passes
  assert_eq!(11, s.capacity()); // fails

  println!("{}", s);

  }
}