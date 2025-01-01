use std::collections::HashMap;

fn main() {
  let input1 = read_file("data/day6_1.txt");

  let result1 = correct_input(input1);
  println!("1: {}", result1);

  let input2 = read_file("data/day6_2.txt");
  let result2 = correct_input(input2);
  println!("2: {}", result2);

  let input1 = read_file("data/day6_1.txt");
  let result3 = correct_input2(input1);
  println!("3: {}", result3);

  let input2 = read_file("data/day6_2.txt");
  let result4 = correct_input2(input2);

  println!("4: {}", result4);
}

fn correct_input2(input: String) -> String {
  let mut result:String="".to_string();

  let mut grid: Vec<Vec<char>> = vec![];

  let mut width = 0;
  let mut height = 0;

  input.split("\n").for_each(|line| {
    let mut lineVec: Vec<char> = vec![];
    width = line.chars().count();
    for c in line.chars() {
      lineVec.push(c);
    }
    grid.push(lineVec);
  });
  height = grid.len();

  for i in 0..width {
    let mut char_map: HashMap<char, i32> = HashMap::new();

    for j in 0..height {
      let c = grid[j][i];
      char_map
          .entry(c)
          .and_modify(|char_count| *char_count += 1)
          .or_insert(1);
    }
    let mut entries: Vec<(&char, &i32)> = char_map.iter().map(|entry| entry).collect();

    entries.sort_by(|a, b| {
      let count_compare = a.1.cmp(b.1);
      count_compare
    });

    println!("{:?}", entries);

    let most=entries.first().unwrap().0;

    result.push_str(&most.to_string());
  }

  result
}

fn correct_input(input: String) -> String {
  let mut result:String="".to_string();

  let mut grid: Vec<Vec<char>> = vec![];

  let mut width = 0;
  let mut height = 0;

  input.split("\n").for_each(|line| {
    let mut lineVec: Vec<char> = vec![];
    width = line.chars().count();
    for c in line.chars() {
      lineVec.push(c);
    }
    grid.push(lineVec);
  });
  height = grid.len();

  for i in 0..width {
    let mut char_map: HashMap<char, i32> = HashMap::new();

    for j in 0..height {
      let c = grid[j][i];
      char_map
          .entry(c)
          .and_modify(|char_count| *char_count += 1)
          .or_insert(1);
    }
    let mut entries: Vec<(&char, &i32)> = char_map.iter().map(|entry| entry).collect();

    entries.sort_by(|a, b| {
      let count_compare = b.1.cmp(a.1);
      count_compare
    });

    println!("{:?}", entries);

    let most=entries.first().unwrap().0;

    result.push_str(&most.to_string());
  }

  result
}

pub fn read_file(file: &str) -> String {
  std::fs::read_to_string(file).unwrap()
}