// 结构体
// struct User {
//   active: bool,
//   username: String,
//   email: String,
//   sign_in_account: u64
// }

fn main() {
  // 注意整个实例必须是可变的；Rust 并不允许只将某个字段标记为可变。
  // 另外需要注意同其他任何表达式一样，我们可以在函数体的最后一个表达式中构造一个结构体的新实例，
  // 来隐式地返回这个实例。
  // let mut user1 = User {
  //   active: true,
  //   username: String::from("somesusername123"),
  //   email: String::from("some@example.com"),
  //   sign_in_account: 1
  // };
  // user1.email = String::from("22222@example.com");
    let width1 = 30;
    let height1 = 50;
    println!(
      "The area of the rectangle is {} square pixels.",
      area(width1, height1)
    )
  
}
fn area(width: u32, height: u32) -> u32 {
  width * height
}

// fn build_user (email: String, username: String) -> User{
//   User {
//     email,
//     username,
//     active: true,
//     sign_in_account:1
//   }
// }