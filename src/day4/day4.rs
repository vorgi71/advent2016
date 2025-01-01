use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
  let input1 = "totally-real-room-200[decoy]";

  println!("{} is valid {}", input1, is_valid_door(input1));

  let input2 = std::fs::read_to_string("data/day4.txt").unwrap();
  let result2 = analyse_input_1(input2);
  println!("Part one: {}", result2);

  let input3 = "qzmt-zixmtkozy-ivhz-343[patyc]";
  let result3 = decrypt_input(input3);
  println!("result3: {:?}", result3);

  let input2 = std::fs::read_to_string("data/day4.txt").unwrap();
  let result4 = decrypt_input(&input2);

  result4.iter().for_each(|x| println!("{}", x));
}

fn decrypt_input(input: &str) -> Vec<String> {
  let mut results: Vec<String> = vec![];

  let a_value = 'a' as u32;

  input.split("\n").for_each(|line| {
    if true {
      if let (Some(start), Some(end)) = (line.rfind('-'), line.find('[')) {
        let room_id = line[start + 1..end].to_string();
        let room_id_value = room_id.parse::<u32>().unwrap();

        let part_to_decrypt = line[0..start].to_string();

        let mut result = "".to_string();
        for c in part_to_decrypt.chars() {
          let new_char = if c == '-' {
            ' '
          } else {
            let new_char_value = (c as u32 - a_value + room_id_value) % 26 + a_value;
            let new_char = char::from_u32(new_char_value).unwrap();

            new_char
          };
          result += new_char.to_string().as_str();
        }
        if result.contains("north") {
          results.push(result+"_"+ &*room_id);
        }
      }
    }
  });

  results
}

fn analyse_input_1(input: String) -> i32 {
  let mut sum = 0;

  input.split("\n").for_each(|line| {
    if is_valid_door(line) {
      if let (Some(start), Some(end)) = (line.rfind('-'), line.find('[')) {
        let room_id = line[start + 1..end].to_string();
        let room_id_value = room_id.parse::<i32>().unwrap();
        sum += room_id_value;
      }
    }
  });

  sum
}

fn is_valid_door(input: &str) -> bool {
  let split = input.split('[').collect::<Vec<_>>()[0];

  let v = split.chars().collect::<Vec<_>>();
  let v = v
      .into_iter()
      .filter(|c| c.is_alphabetic())
      .collect::<Vec<_>>();

  let mut char_map: HashMap<char, i32> = HashMap::new();
  for c in v {
    char_map
        .entry(c)
        .and_modify(|char_count| *char_count += 1)
        .or_insert(1);
  }

  let mut entries: Vec<(&char, &i32)> = char_map.iter().map(|entry| entry).collect();

  entries.sort_by(|a, b| {
    let count_compare = b.1.cmp(a.1);
    if count_compare == Ordering::Equal {
      a.0.cmp(b.0)
    } else {
      count_compare
    }
  });

  println!("{:?}", entries);

  let code = entries[0..5].iter().map(|&c| c.0).collect::<String>();

  let mut code_from_input: &str = "?";
  if let (Some(start), Some(end)) = (input.find('['), input.find(']')) {
    code_from_input = &input[start + 1..end];
  }

  println!("{} {} = {}", code, code_from_input, code == code_from_input);

  code_from_input == code
}
