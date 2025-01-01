pub fn read_file(file: &str) -> String {
  std::fs::read_to_string(file).unwrap()
}

struct KeyPad {
  keys: Vec<Vec<char>>,
}

impl KeyPad {
  fn new() -> KeyPad {
    KeyPad {
      keys: vec![
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
      ],
    }
  }

  fn get(&self, x: usize, y: usize) -> char {
    let row = self.keys.get(y);
    if row.is_some() {
      let char = row.unwrap().get(x);
      if char.is_some() {
        return *char.unwrap();
      }
    }
    ' '
  }

  fn from_str(str: &str) -> KeyPad {
    let mut vec: Vec<Vec<char>> = Vec::new();
    str.split("\n").for_each(|line| {
      let line_vec = line.chars().collect::<Vec<char>>();
      vec.push(line_vec);
    });
    KeyPad { keys: vec }
  }
}

pub enum Direction {
  North,
  East,
  South,
  West,
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Pos {
  x: i32,
  y: i32,
}

impl Direction {
  fn turn_left(&self) -> Direction {
    match self {
      Direction::North => Direction::West,
      Direction::West => Direction::South,
      Direction::South => Direction::East,
      Direction::East => Direction::North,
    }
  }
  fn turn_right(&self) -> Direction {
    match self {
      Direction::North => Direction::East,
      Direction::East => Direction::South,
      Direction::South => Direction::West,
      Direction::West => Direction::North,
    }
  }

  fn plus(&self, pos: &Pos) -> Pos {
    match self {
      Direction::North => Pos {
        x: pos.x,
        y: pos.y - 1,
      },
      Direction::East => Pos {
        x: pos.x + 1,
        y: pos.y,
      },
      Direction::South => Pos {
        x: pos.x,
        y: pos.y + 1,
      },
      Direction::West => Pos {
        x: pos.x - 1,
        y: pos.y,
      },
    }
  }
}

fn main() {
  let input1 = "ULL
RRDDD
LURDL
UUUUD";

  let result1 = process_input(&input1);

  println!("{}", result1);

  let input2 = read_file("data/day2/day2.txt");
  let result2 = process_input(&input2);
  println!("{}", result2);

  let result3 = process_input_2(&input1);
  println!("{}", result3);

  let result4 = process_input_2(&input2);
  println!("{}", result4);
}

fn process_input(input: &str) -> String {
  let mut code = "".to_string();
  let mut pos = Pos { x: 1, y: 1 };

  let key_pad = KeyPad::new();

  input.split("\n").for_each(|line| {
    line.chars().for_each(|c| {
      match c {
        'U' => pos.y -= 1,
        'D' => pos.y += 1,
        'L' => pos.x -= 1,
        'R' => pos.x += 1,
        _ => {}
      }
      if (pos.x < 0) {
        pos.x = 0
      }
      if (pos.x > 2) {
        pos.x = 2
      }

      if (pos.y < 0) {
        pos.y = 0
      }
      if (pos.y > 2) {
        pos.y = 2
      }
    });
    let key = key_pad.keys[pos.y as usize][pos.x as usize];
    code.push(key)
  });

  code
}

fn process_input_2(input: &str) -> String {
  let mut code = "".to_string();
  let mut pos = Pos { x: 0, y: 2 };

  let key_pad = KeyPad::from_str("  1  \n 234 \n56789\n ABC \n  D  ");

  input.split("\n").for_each(|line| {
    line.chars().for_each(|c| {
      let old_pos = pos.clone();

      match c {
        'U' => pos.y -= 1,
        'D' => pos.y += 1,
        'L' => pos.x -= 1,
        'R' => pos.x += 1,
        _ => {}
      }
      let key = key_pad.get(pos.x as usize, pos.y as usize);
      if key == ' ' {
        pos = old_pos;
      }
    });
    let key = key_pad.get(pos.x as usize, pos.y as usize);
    code.push(key);
  });

  code
}
