use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
  println!("Guess the number!");
  println!("Please input your guess.");
  // mut 可变变量
  let mut guess = String::new();
  let secret_number = rand::thread_rng().gen_range(1..10);
  // println!("The secret number is {}", secret_number);
  // let a = 33;
  loop {
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read input");
    println!("You guess: {}", guess);
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
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