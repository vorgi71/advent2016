mod utils;
use utils::utils::Pos;
use utils::utils::Direction;

pub fn read_file(file: &str) -> String {
  std::fs::read_to_string(file).unwrap()
}

fn main() {
  let input1="R5, L5, R5, R3".to_string();

  let result1=process_input(input1);
  println!("result1: {}", result1);

  let input2=read_file("data/day1/day1.txt");

  let result2=process_input(input2.clone());
  println!("result2: {}", result2);

  let input3="R8, R4, R4, R8".to_string();

  let result3=process_input2(input3);
  println!("result3: {}", result3);

  let result4=process_input2(input2);

  println!("result4: {}", result4);

  println!("Day 1")
}

fn process_input(input: String) -> i32 {
  let commands:Vec<&str>=input.split(", ").collect();
  let mut position = Pos{x: 0, y: 0};
  let mut direction = Direction::North;

  for command in commands {
    if command.starts_with("L") {
      direction = direction.turn_left();
    } else if command.starts_with("R") {
      direction = direction.turn_right();
    }
    let moves=command[1..].parse::<i32>().unwrap();
    for _ in 0..moves {
      position = position.plus(&direction);
    }
  }

  position.x.abs() + position.y.abs()
}

fn process_input2(input: String) -> i32 {
  let commands:Vec<&str>=input.split(", ").collect();
  let mut position = Pos{x: 0, y: 0};
  let mut visited:Vec<Pos> = vec![position];
  let mut direction = Direction::North;

  for command in commands {
    if command.starts_with("L") {
      direction = direction.turn_left();
    } else if command.starts_with("R") {
      direction = direction.turn_right();
    }
    let moves=command[1..].parse::<i32>().unwrap();
    for _ in 0..moves {
      let new_position = position.plus(&direction);
      if visited.contains(&new_position) {
        println!("Duplicate position found");
        return new_position.x.abs() + new_position.y.abs()
      }
      visited.push(new_position);
      position = new_position;
    }
  }
  -1
}