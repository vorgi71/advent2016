mod utils;

use std::str::Chars;
use md5::{Digest, Md5};
use crate::utils::utils::{CharGrid, Direction, Pos};
use crate::utils::utils::Direction::{North, East, West, South};

fn main() {
  let input1 = "ulqzkmiv";

  let result1=calculate_path(input1);
  println!("result1 = {}", result1);
  
  assert_eq!(result1, "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
  
  let result2 = calculate_path("mmsxrhfx");
  println!("result2 = {}", result2);
}

fn calculate_path(input: &str) -> String {
  let path:Vec<char>=input.chars().collect();
  let pos=Pos {x:0,y:0};

  let mut stack:Vec<(Vec<char>,Pos)>=Vec::new();
  stack.push((path,pos));
  
  let mut solutions:Vec<String>=vec![];
  let mut min_solution=0x7FFFFFFF;

  while let Some((path,pos))=stack.pop() {
    println!("{:?}{:?}",pos,path);
    if pos.x == 3 && pos.y == 3 {
      let solution=path.iter().collect::<String>();
      let len=solution.len();
      if len<min_solution {
        let input_len=input.len();
        let solution = solution[input_len..].to_string();
        min_solution=len;
        solutions.push(solution);
      }
    }
    if path.len()>=min_solution {
      continue;
    }
    let mut hasher = Md5::new();
    hasher.update(path.iter().collect::<String>());
    let result = hasher.finalize();
    let result = format!("{:x}", result);
    let possible_directions=get_directions(pos,result[0..4].chars().collect());
    println!("{:?}",possible_directions);
    for direction in possible_directions {
      let mut new_path = path.clone();
      match direction {
        North => new_path.push('U') ,
        South => new_path.push('D') ,
        West => new_path.push('L') ,
        East => new_path.push('R') ,
      }
      let new_pos = pos.clone().plus(&direction);
      stack.push((new_path,new_pos));
    }
  }

  
  solutions.sort_by_key(|a| a.len());
  println!("{:?}",solutions);
  
  let best_solution=solutions.first();
  
  best_solution.unwrap().to_string()
}


fn get_directions(pos: Pos, chars: Vec<char>) -> Vec<Direction> {
  let mut north =true;
  let mut south =true;
  let mut west =true;
  let mut east =true;
  
  if pos.x==0 { west=false };
  if pos.x==3 { east=false };
  if pos.y==0 { north=false }
  if pos.y==3 { south=false };
  
  fn test(c: char) -> bool {
    match c {
      'b' | 'c' | 'd' | 'e' | 'f' => true,
      _ => false, // Important: handle the default case
    }
  }
  
  if !test(chars[0]) { north = false; } 
  if !test(chars[1]) { south = false; } 
  if !test(chars[2]) { west = false; } 
  if !test(chars[3]) { east = false; } 

  let mut result = vec![];
  if north { result.push(North) }
  if south { result.push(South) }
  if east { result.push(East) }
  if west { result.push(West) }
  
  result
}