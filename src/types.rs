/*
Primitive Types--
  Integers: u8, u16, i16, u32, i32, u64, i64, u128, i128...
  ... number of bits they take in memory

  Floats: f32, f64
  Boolean (bool)
  Characters (char)
  Tuples
  Arrays

Rust is a statically typed language, which means that it must know
the types of all variables at compile time, however, the compiler
can usually infer what type we want to use based on the value and how we use it.

*/

pub fn run() {
  // Defaults to "i32"
  let _x = 1;

  // Defaults to "f64"
  let _y = 2.5;

  // Explicit Type
  let _z: i64 = 434343434343443;

  // Find Max size
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  // Boolean
  let is_active = true; // or let is_active: bool = true;

  // Get Boolean from expression
  let is_greater = 10 > 1;

  // Char
  let a1 = 'a'; // Char's need single quotes
  let face = '\u{1f600}';

  println!("{:?}", (_x, _y, _z, is_active, is_greater, a1, face));

}