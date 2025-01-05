mod utils;

use std::ops::{Range, RangeInclusive};
use utils::utils::read_lines;

fn main() {
  let input1 = read_lines("data/day20_1.txt");
  let result1 = find_valid_ip(input1);
  println!("result1: {:?}", result1);
}

fn find_valid_ip(input: Vec<String>) -> u32 {
  let mut blacklist:Vec<RangeInclusive<u32>>=Vec::new();
  
  for line in input {
    let split= line.split("-").collect::<Vec<&str>>();
    let range = (split[0].parse::<u32>().unwrap() ..= split[1].parse::<u32>().unwrap());
    blacklist.push(range);
  }
  
  blacklist.sort_by(|a, b| a.start().cmp(&b.start()));
  let min = *blacklist[0].start();
  let max= *blacklist.iter().max_by(|a,b| a.end().cmp(&b.end())).unwrap().start();
  
  for i in min..=max {
      let mut contains=true;
    for range in blacklist.iter() {
      contains=true;
      if range.contains(&i) {
        contains=false;
        break;
      }
    }
    if !contains {
      return i;
    }
  }
  
  0
}