use crate::utils::utils::{CharGrid, Pos};
use crate::utils::utils::Direction::North;
use crate::utils::utils::Direction::East;
use crate::utils::utils::Direction::South;
use crate::utils::utils::Direction::West;

mod utils;

fn main() {
  let input1 = 10;
  let grid1 = create_grid(input1, 10, 10);
  println!("{}", grid1);
  let result1 = solve_grid(grid1,Pos {x:7,y:4});
  println!("{}", result1);
  let result1_2 = fill_grid(create_grid(input1, 15, 15), 1);
  println!("{}", result1_2);

  let input2 = 1364;
  let grid2 = create_grid(input2, 60, 60);
  println!("{}", grid2);
  let result2 = solve_grid(grid2,Pos {x:31,y:39});
  println!("{}", result2);
  let result2_2 = fill_grid(create_grid(input2, 70, 70), 50);
  println!("{}", result2_2);
}

fn fill_grid(mut grid: CharGrid, max_moves: usize) -> usize {
  let start_point = Pos {x:1,y:1};

  let mut visited_points: Vec<(Pos,usize)> = vec![(start_point,0)];

  let mut point_stack: Vec<(Pos,usize)> = vec![(start_point,0)];
  
  while let Some((pos, steps)) = point_stack.pop() {
    for dir in [North, South, East, West] {
      let new_pos=pos.plus(&dir);
      let new_steps=steps+1;
      if new_steps== max_moves+1 {
        continue;
      }
      if grid.get(new_pos.x as usize, new_pos.y as usize).is_some() 
          && grid.get(new_pos.x as usize, new_pos.y as usize).unwrap()=='.' 
          && !visited_points.iter().any(|&(pos, _)| pos == new_pos) {
        point_stack.push((new_pos, steps + 1));
        visited_points.push((new_pos, steps + 1));
      }
    }
  }

  visited_points.iter().for_each(|(point,_)| {
    grid.set(point.x as usize, point.y as usize, 'O');
  });
  
  println!("filled:");
  println!("{}", grid);
  println!();
  
  visited_points.len()
}

fn solve_grid(mut grid: CharGrid, endpoint: Pos) -> usize {
  let start_point = Pos {x:1,y:1};

  let mut visited_points: Vec<(Pos,usize)> = vec![(start_point,0)];

  let mut point_stack: Vec<(Pos,usize)> = vec![(start_point,0)];
  
  while let Some((point,moves)) = point_stack.pop() {
    for dir in [North, East, South, West] {
      if point==endpoint {
        visited_points.push((point,moves+1));
        continue 
      };
      
      let new_point=point.plus(&dir);
      
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
  
  visited_points.iter().for_each(|(point,_)| {
    grid.set(point.x as usize, point.y as usize, 'O');
  });
  grid.set(endpoint.x as usize, endpoint.y as usize, 'E');
  
  println!("{grid}");
  
  if let Some(&(found, foundmoves)) = visited_points.iter().find(
    |(pos, moves)| 
      { *pos == endpoint }
  ) {
    println!("found:{:?}", found);
    return foundmoves;
  };
  
  0usize
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
