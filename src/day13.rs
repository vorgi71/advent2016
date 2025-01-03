use crate::utils::utils::{CharGrid, Direction, Pos};
use crate::utils::utils::Direction::North;
use crate::utils::utils::Direction::East;
use crate::utils::utils::Direction::South;
use crate::utils::utils::Direction::West;

mod utils;

fn main() {
  let input1 = 10;
  let grid1 = create_grid(input1, 10, 10);
  println!("{}", grid1);
  let result1 = solve_grid(grid1,Pos {x:7,y:7});
  println!("{}", result1);

  let input2 = 1364;
  let grid2 = create_grid(input2, 40, 40);
  println!("{}", grid2);
}

fn solve_grid(grid: CharGrid, endpoint: Pos) -> i32 {
  let start_point = Pos {x:1,y:1};

  let mut visited_points: Vec<(Pos,usize)> = vec![(start_point,0)];

  let mut point_stack: Vec<(Pos,usize)> = vec![(start_point,0)];
  
  while let Some((point,moves)) = point_stack.pop() {
    for dir in [North, East, South, West] {
      let new_point=point.plus(&dir);
      if new_point==endpoint {
        break;
      }
      
      let char_at=grid.get(new_point.x as usize, new_point.y as usize);
      if char_at.is_some() && char_at.unwrap() == '.' {
        if let Some(index) = visited_points
            .iter()
            .position(|(p,m)| {*p==new_point}) {
          if visited_points[index].1>moves+1 {
            visited_points[index]= (new_point,moves+1);
            point_stack.push((new_point,moves+1));
          }
        } else {
          visited_points.push((new_point,moves+1));
          point_stack.push((new_point,moves+1));
        }
      }
    }
  }

  println!("visited_points:{:?}", visited_points);
  println!("point_stack:{:?}", point_stack);
  
  0
}

fn create_grid(input: i32, width: usize, height: usize) -> CharGrid {
  let mut char_grid = CharGrid::new(width, height, '.');

  for y in 0..height {
    for x in 0..width {
      let result = x * x + 3 * x + 2 * x * y + y + y * y + input as usize;
      if result.count_ones() % 2 == 1 {
        char_grid.set(x, y, '*');
      };
    }
  }

  char_grid
}
