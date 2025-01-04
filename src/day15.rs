mod utils;

use regex::Regex;
use utils::utils::read_lines;

fn main() {
  let input1 = read_lines("data/day15_1.txt");
  let result1=process_input(input1);
  println!("Part 1: {}",result1);
  
  let input2 = read_lines("data/day15_2.txt");
  let result2=process_input(input2);
  println!("Part 2: {}",result2);
  
  let mut input3 = read_lines("data/day15_2.txt");
  input3.push("Disc #7 has 11 positions; at time=0, it is at position 0.".to_string());
  let result3=process_input(input3);
  println!("Part 3: {}",result3);
}

#[derive(Debug)]
struct Disc {
  id: u32,
  positions: u32,
  position: u32,
}

impl Disc {
  fn align(&mut self) {
    self.position += self.id;
    self.position%=self.positions;
  }
  
  fn step(&mut self) {
    self.position+=1;
    self.position%=self.positions;
  }
}

fn parse_disc(line: &str) -> Option<Disc> {
  let re = Regex::new(r"^Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+)\.$").unwrap();
  if let Some(captures) = re.captures(line) {
    let id = captures.get(1)?.as_str().parse().ok()?;
    let positions = captures.get(2)?.as_str().parse().ok()?;
    let initial_position = captures.get(3)?.as_str().parse().ok()?;
    Some(Disc { id, positions, position:initial_position })
  } else {
    None
  }
}

fn process_input(input: Vec<String>) -> usize {
  let mut discs: Vec<Disc> = input.iter().map(|line| {
    parse_disc(line).unwrap()
  }).collect();
  
  discs.iter_mut().for_each(|mut disc| disc.align());
  
  let mut time=0;
  loop {
    if discs.iter().all(|disc| disc.position==0) {
      break;
    }
    discs.iter_mut().for_each(|disc| disc.step());
    time+=1;
  }
  
  println!("{:?}",discs);
  
  time
}