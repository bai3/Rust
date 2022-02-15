use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
  println!("Guess the number!");

  // mut 可变变量
  let secret_number = rand::thread_rng().gen_range(1..10);
  // println!("The secret number is {}", secret_number);
  // let a = 33;
  loop {
    let mut guess = String::new();
    println!("Please input your guess.");
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read input");
    let guess: u32 = match guess.trim().parse() {
      Ok(x) => x,
      Err(_) => continue,
    };
    println!("Your guess nmber if {}",guess);
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small"),
      Ordering::Greater => println!("Too big"),
      Ordering::Equal => {
        println!("You win");
        break;
      }
    }
  }
}