pub mod utils {
  use std::fmt::{Display, Formatter};

  pub fn read_file(file: &str) -> String {
    std::fs::read_to_string(file).unwrap()
  }

  pub fn read_lines(file: &str) -> Vec<String> {
    read_file(file).lines().map(String::from).collect()
  }

  pub struct CharGrid {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<char>>,
  }

  impl Display for CharGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      for row in &self.data {
        for &c in row {
          write!(f, "{}", c)?;
        }
        writeln!(f)?; // Add a newline after each row
      }
      write!(f, "")
    }
  }

  impl CharGrid {
    pub fn new(width:usize,height:usize,init_value:char) -> CharGrid {
      CharGrid {
        width,height,data: vec![vec![init_value; width]; height]
      }
    }

    pub fn get(&self, x:usize, y:usize) -> Option<char> {
      if y >= self.height || x >= self.width {
        return None;
      }
      Some(self.data[y][x])
    }
    pub fn set(&mut self, x:usize, y:usize, new_value:char) {
      if y < self.height || x < self.width {
        self.data[y][x] = new_value;
      }
    }
  }

  #[derive(Debug)]
  pub enum Direction {
    North,
    East,
    South,
    West,
  }

  #[derive(Eq, PartialEq, Copy, Clone, Debug)]
  pub struct Pos {
    pub x: i32,
    pub y: i32,
  }

  impl Direction {
    pub fn turn_left(&self) -> Direction {
      match self {
        Direction::North => Direction::West,
        Direction::West => Direction::South,
        Direction::South => Direction::East,
        Direction::East => Direction::North,
      }
    }
    pub fn turn_right(&self) -> Direction {
      match self {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
      }
    }

    fn plus(&self, pos: &Pos) -> Pos {
      match self {
        Direction::North => { Pos {x: pos.x , y: pos.y-1} }
        Direction::East => { Pos {x: pos.x+1, y: pos.y} }
        Direction::South => { Pos {x: pos.x , y: pos.y+1} }
        Direction::West => { Pos {x: pos.x-1, y: pos.y} }
      }
    }
  }

  impl Pos {
    pub fn plus(&self, dir:&Direction) -> Pos {
      match dir {
        Direction::North => { Pos {x: self.x , y: self.y-1} }
        Direction::East => { Pos {x: self.x+1, y: self.y} }
        Direction::South => { Pos {x: self.x , y: self.y+1} }
        Direction::West => { Pos {x: self.x-1, y: self.y} }
      }
    }
  }

}