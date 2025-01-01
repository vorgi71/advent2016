use md5::{Digest, Md5};
use std::usize;

fn part1() {

  let mut index=0i64;
  let mut code = "".to_string();
  while code.len()<8 {
    let mut hasher = Md5::new();

    // Hash a string
    hasher.update(format!("uqwqemis{}",index));

    // Get the hash result as a byte array
    let result = hasher.finalize();

    let hashcode = format!("{:x}", result);

    if hashcode.starts_with("00000") {
      let substring=hashcode[5..6].to_string();
      println!("substring: {} index: {}", substring, index);
      code+= &*substring;
    }
    index+=1;
  }

  println!("{}",code);
}

fn part2() {
  let mut index=0i64;
  let mut code = vec!['_';8];
  while code.iter().any(|c| *c=='_') {
    let mut hasher = Md5::new();

    // Hash a string
    hasher.update(format!("uqwqemis{}",index));

    // Get the hash result as a byte array
    let result = hasher.finalize();

    let hashcode = format!("{:x}", result);

    if hashcode.starts_with("00000") {
      let substring=hashcode[6..7].to_string();
      let index=usize::from_str_radix(&hashcode[5..6], 16).unwrap();
      if index<=7 {
        if(code[index]=='_') {
          code[index] = substring.chars().nth(0).unwrap();
          println!("{:?}", code);
        }
      }
    }
    index+=1;
  }

  println!("{:?}",code);

}

fn main() {
  part2();
}