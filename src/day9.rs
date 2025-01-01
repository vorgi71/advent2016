mod utils;
use utils::utils::read_lines;

fn main() {
  let input1 = read_lines("./data/day9_1.txt");
  let result1 = unzip_lines(&input1);
  println!("result1: {}", result1);

  let input2 = read_lines("./data/day9_2.txt");
  let result2 = unzip_lines(&input2);
  println!("result2: {}", result2);

  let result3 = get_unzip_size(input1);
  println!("result3: {}", result3);

  let result4 = get_unzip_size(input2);
  println!("result4: {}", result4);
}

fn get_unzip_size(input: Vec<String>) -> u128 {
  let mut sum: u128 = 0;

  for line in input {
    let unzipped = get_line_size(&line);
    sum += unzipped;
  }

  sum
}

fn get_line_size(line: &str) -> u128 {
  let mut sum: u128 = 0;

  if !line.contains("(") {
    return line.len() as u128;
  }

  let start_bracket = line.find('(').unwrap();
  sum += start_bracket as u128;

  if let Some(end_bracket) = line.find(')') {
    let command_string = &line[start_bracket + 1..end_bracket];
    let split = command_string.split("x").collect::<Vec<&str>>();
    let num_chars = split[0].parse::<usize>().unwrap();
    let reps = split[1].parse::<u32>().unwrap();
    let rep_string = &line[end_bracket + 1..end_bracket + 1 + num_chars];

    sum += reps as u128 * get_line_size(rep_string);

    sum += get_line_size(&line[end_bracket + 1 + num_chars..])
  }

  sum
}

fn unzip_lines(input: &Vec<String>) -> u32 {
  let mut sum: u32 = 0;

  for line in input {
    let unzipped = unzip_line(&line);
    sum += unzipped.len() as u32;
  }

  sum
}

fn unzip_line(line: &str) -> String {
  let mut result = String::new();

  let mut index = 0;

  while index < line.len() {
    let next = &line[index..index + 1];
    if next == "(" {
      if let Some(end_bracket) = line[index..].find(")") {
        let end_bracket = index + end_bracket;
        let substring = &line[index + 1..end_bracket];
        let split = substring.split("x").collect::<Vec<&str>>();
        let num_chars = split[0].parse::<usize>().unwrap();
        let reps = split[1].parse::<u32>().unwrap();
        let rep_string = &line[end_bracket + 1..end_bracket + 1 + num_chars];
        for i in 0..reps {
          result.push_str(&rep_string);
        }
        index = end_bracket + num_chars;
      }
    } else {
      result.push_str(next);
    }
    index += 1;
  }

  println!("result: {}", result);

  result
}