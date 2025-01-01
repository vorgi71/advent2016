use std::collections::HashMap;
use crate::utils::utils::read_lines;

mod utils;

#[derive(Debug)]
#[derive(Clone)]
struct Bot {
  name: String,
  chips: Vec<i32>,
  instruction: Option<Instruction>,
}

impl Bot {
  fn new(name: String) -> Bot {
    Bot {
      name,
      chips: vec![],
      instruction: None,
    }
  }
}

#[derive(Debug)]
#[derive(Clone)]
struct Instruction {
  give_low_to: String,
  give_high_to: String,
}

impl Instruction {
  fn new() -> Instruction {
    Instruction {
      give_low_to: String::from(""),
      give_high_to: String::from(""),
    }
  }
}

fn main() {
  let input1 = read_lines("data/day10_1.txt");
  let result1 = process_instructions(&input1);
  println!("result1:{}", result1);

  let input2 = read_lines("data/day10_2.txt");
  let result2 = process_instructions(&input2);
  println!("result2:{}", result2);
}

fn process_instructions(lines: &Vec<String>) -> String {
  let mut bot_map: HashMap<String, Bot> = HashMap::new();
  for line in lines {
    if line.starts_with("value") {
      let end_value = line.find(" goes").unwrap();
      let value_str = &line[6..end_value];
      let value = value_str.parse::<i32>().unwrap();
      let bot_start = line.find("to ").unwrap();
      let bot_name = &line[bot_start + 3..];
      let bot = bot_map.entry(bot_name.to_string()).
          or_insert(Bot::new(bot_name.to_string()));

      bot.chips.push(value);
      if bot.chips.len() > 2 {
        panic!("Too many chips for {:?}", bot);
      }

      println!("bot {:?}", bot);
    } else {
      let bot1_end = line.find(" gives").unwrap();
      let bot_name = &line[0..bot1_end];
      let low_to_start = line.find(" low to ").unwrap() + " low to ".len();
      let low_to_end = line.find(" and high to").unwrap();
      let bot_low = line[low_to_start..low_to_end].to_string();
      let bot_high_start = line.find("high to").unwrap() + "high to ".len();
      let bot_high = line[bot_high_start..].to_string();

      let bot = bot_map.entry(bot_name.to_string()).
          or_insert(Bot::new(bot_name.to_string()));
      if bot.instruction.is_none() {
        bot.instruction = Some(Instruction {
          give_high_to: bot_high,
          give_low_to: bot_low,
        });
      } else {
        panic!("instruction already set for {:?}", bot);
      }
      println!("bot {:?}", bot);
    }
  }

  process_bot_map(bot_map);

  "?".to_string()
}

fn process_bot_map(mut bot_map: HashMap<String, Bot>) {
  let mut changed = true;
  while changed {
    changed = false;

    // Collect the *names* of bots with two chips
    let bot_names_with_two_chips: Vec<String> = bot_map.iter()
        .filter(|(_, bot)| bot.chips.len() == 2)
        .map(|(name, _)| name.to_string())
        .collect();

    // Process each bot with two chips
    for bot_name in bot_names_with_two_chips {
      if let Some(bot) = bot_map.get(&bot_name.to_string()).cloned() {
        if let Some(instruction) = &bot.instruction {
          let low_bot_name = instruction.give_low_to.clone();
          let high_bot_name = instruction.give_high_to.clone();
          let low_chip = if bot.chips[0] < bot.chips[1] { bot.chips[0] } else { bot.chips[1] };
          let high_chip = if bot.chips[0] > bot.chips[1] { bot.chips[0] } else { bot.chips[1] };

          if let Some(low_bot) = bot_map.get_mut(&low_bot_name) {
            low_bot.chips.push(low_chip);
            changed = true;
            println!("bot {} gets low chip {}", low_bot_name, low_chip);

          }
          if let Some(high_bot) = bot_map.get_mut(&high_bot_name) {
            high_bot.chips.push(high_chip);
            changed = true;
            println!("bot {} gets high chip {}", high_bot_name, high_chip);
          }
        }
      }
      if let Some(bot) = bot_map.get_mut(&bot_name.to_string()) {
        bot.chips.clear();
      }
    }
    let found = bot_map.iter().find(|&(_, bot)| {
      bot.chips.contains(&61) && bot.chips.contains(&17)
    });
    if let Some((_,bot)) = found {
      println!("found {:?}", bot);
    }
  }
}