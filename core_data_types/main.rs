#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

fn main() {
  let a: u8 = 123; // u = unsigned, 8bits, 0-255
  // it's a macro because of the exclaimation mark
  // pair of curly braces will be replaced with the value of the variable
  println!("a = {}", a); // immutable

  // `mut` is short for mutable
  // i = signed -2^(n-1) .. 2^(n-1) - 1
  let mut b: i8 = 0; // mutable, i8 = signed, 8bits, -128-127
  println!("b = {} before", b); // mutable
  b = 42; // mutable
  println!("b = {} after", b); // mutable

  // type inference
  let mut c = 123456789; // c = i32, the rust compiler figures out what type to use
  println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));
  c = -1;
  println!("c = {}", c);

  // u8, u15, u32, u64, u128, i8, i15, i32, i64, i128
  // usize, isize
  let z: isize = 123;
  let size_of_z: usize = mem::size_of_val(&z);
  println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

  // character
  let d: char = 'x';
  println!("{} is a character, takes up {} bytes", d, mem::size_of_val(&d));

  // f32, f64 ieee-754
  let e: f32 = 2.5;
  println!("e = {}, takes up {} bytes", e, mem::size_of_val(&e));

  // boolean
  let g = true;
  println!("g = {}, takes up {} bytes", g, mem::size_of_val(&g));
}