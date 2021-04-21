fn main() {
    let a_number = 10;
    let a_boolean = true;
  
    println!("the number is {}.", a_number);
    println!("the boolean is {}.", a_boolean);

  let mut a_number = 10; // 默认不可变的得将变量标记为mut
  println!("the number is {}.", a_number);
  a_number = 15;
  println!("and now the number is {}.", a_number);
}
