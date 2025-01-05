mod utils;

use std::ops::{Range, RangeInclusive};
use utils::utils::read_lines;

fn main() {
  let input1 = read_lines("data/day20_1.txt");
  let result1 = find_valid_ip(&input1);
  println!("result1: {:?}", result1);

  let input2 = read_lines("data/day20_2.txt");
  // let result2=find_valid_ip(&input2);
  // println!("result2: {:?}", result2);

  let result3=find_valid_ips(input1,9);
  println!("result3: {:?}", result3);
  
  let result4 = find_valid_ips(input2,4294967295);
  println!("result4: {:?}", result4);
}

fn find_valid_ip(input: &Vec<String>) -> u32 {
  let mut blacklist:Vec<RangeInclusive<u32>>=Vec::new();

  for line in input {
    let split= line.split("-").collect::<Vec<&str>>();
    let range = (split[0].parse::<u32>().unwrap() ..= split[1].parse::<u32>().unwrap());
    blacklist.push(range);
  }

  blacklist.sort_by(|a, b| a.start().cmp(b.start()));
  let min = *blacklist[0].start();
  let max= *blacklist.iter().max_by(|a,b| a.end().cmp(b.end())).unwrap().end();

  for i in min..=max {
      let mut contains=true;
    for range in blacklist.iter() {
      contains=false;
      if range.contains(&i) {
        contains=true;
        break;
      }
    }
    if !contains {
      return i;
    }
  }

  0
}

fn find_valid_ips(input: Vec<String>, max:u32) -> u32 {
  let mut blacklist:Vec<RangeInclusive<u32>>=Vec::new();

  for line in input {
    let split= line.split("-").collect::<Vec<&str>>();
    let range = (split[0].parse::<u32>().unwrap() ..= split[1].parse::<u32>().unwrap());
    blacklist.push(range);
  }

  blacklist.sort_by(|a, b| a.start().cmp(&b.start()));
  let min = 0;

  let mut sum =0u32;

  for i in min..=max {
    if i%1_000_000==0 {
      println!("{i} {}",blacklist.len());
    }
    
    let mut marked_for_removal:Vec<u32>=Vec::new();
    
    let mut contains=true;
    for range_index in 0..blacklist.len() {
      let range = &blacklist[range_index];
      if range.end()< &i {
        marked_for_removal.push(range_index as u32);
      }
      
      contains=false;
      if range.contains(&i) {
        contains=true;
        break;
      }
    }
    if !contains {
      sum+=1;
    }
    if !marked_for_removal.is_empty() {
      for remove_index in marked_for_removal {
        blacklist.remove(remove_index as usize);
      }
    }
  }

  sum
}