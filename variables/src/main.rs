fn main() {
  // wrong
  let mut x = 5;
  const MAS:u32 = 333;
  println!("x: {}", x);
  println!("MAS: {}", MAS);
  x = 6;
  println!("x: {}", x);
}
