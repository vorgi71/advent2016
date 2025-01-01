
fn main() {
  let input1 = "jdpqymeesiieeeb[iwrygkpzdjttxuz]qqrbobabyedbigesuh[tmfkwpopdgcfuydhukb]mzldoxgjdeckpdvu[obojbnmmlhdwsman]nssaclvkjwmoozuissi";
  let result1 = check_abba(input1);
  println!("result1: {}", result1);

  let input2 = std::fs::read_to_string("data/day7.txt").unwrap();
  let result2 = check_abba(&input2);
  println!("result2: {}", result2);

  let input3 = "xyx[xyx]xyx";
  let result3 = check_bab(input3);
  println!("result3: {}", result3);

  let result4 = check_bab(&input2);
  println!("result4: {}", result4);
}

fn check_bab(input: &str) -> i32 {
  let mut sum=0;
  input.split("\n").for_each(|line| {
    if check_bab_line(line) {sum+=1};
  });

  sum
}

fn check_bab_line(line: &str) -> bool {
  let mut outsides: Vec<&str> = Vec::new();
  let mut insides: Vec<&str> = Vec::new();

  let mut start = 0;
  while let Some(open_bracket) = line[start..].find('[') {
    let open_bracket_index = start + open_bracket;

    if open_bracket_index > start {
      outsides.push(&line[start..open_bracket_index]);
    }

    if let Some(close_bracket) = line[open_bracket_index + 1..].find(']') {
      let close_bracket_index = open_bracket_index + 1 + close_bracket;
      insides.push(&line[open_bracket_index + 1..close_bracket_index]);
      start = close_bracket_index + 1;
    } else {
      // Handle unmatched bracket (optional: return an error, break, etc.)
      outsides.push(&line[open_bracket_index+1..]);
      break;
    }
  }

  if start < line.len() {
    outsides.push(&line[start..]);
  }

  for outside in outsides {
    let chars= outside.chars().collect::<Vec<char>>();
    for index in 0..chars.len()-2 {
      if chars[index] == chars[index+2] {
        let bab=format!("{}{}{}",chars[index+1],chars[index],chars[index+1]);
        if insides.iter().any(|inside| { inside.contains(&bab) }) {
          return true
        }
      }
    }
  }

  false
}

fn check_abba(input: &str) -> i32 {
  fn is_abba(chars: Vec<char>) -> bool {
    let index = 0;

    if chars[index] == chars[index + 3]
        && chars[index + 1] == chars[index + 2]
        && chars[index] != chars[index + 1]
    {
      if chars[index..index + 4].iter().any(|c| c.is_alphabetic()) {
        return true;
      }
    }
    false
  }

  let mut sum = 0;
  input.lines().for_each(|line| {
    let chars: Vec<char> = line.chars().collect();
    let mut index = 0;
    let mut abba = false;
    let mut in_brackets = false;
    while index < chars.len() - 3 {
      if in_brackets {
        if chars[index] == ']' {
          in_brackets = false;
        } else {
          if is_abba(chars[index..index + 4].to_vec()) {
            abba = false;
            break;
          }
        }
      } else {
        if chars[index] == '[' {
          in_brackets = true;
        } else {
          if is_abba(chars[index..index + 4].to_vec()) {
            abba = true;
          }
        }
      }
      index += 1;
    }
    if abba {
      println!("{}", line);
      sum += 1;
    }
  });

  sum
}
