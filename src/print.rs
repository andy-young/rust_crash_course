pub fn run() {
  // Print to console
  println!("Hello form the print.rs file.");

  // Basic Formatting
  println!("{} is the loneliest number, {}, is not so.", 1, 2);

  // Positional Arguments
  println!("{0} is a {1} cat who {2} {3}!", "vony", "loving", "eats", "treats");

  // Named Arguments
  println!("{hobbie} is something {name} likes to do.", name = "Andy", hobbie = "Mycology");

  // Placeholder Traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

  // Placeholder for debug trait
  println!("{:?}", (12, false, "hello"));

  
}
