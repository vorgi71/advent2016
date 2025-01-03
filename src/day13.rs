use crate::utils::utils::CharGrid;

mod utils;

fn main() {
  let input1=10;
  let grid1 =create_grid(input1, 10, 10);
  println!("{}", grid1);
  
  let input2=1364;
  let grid2 =create_grid(input2, 40, 40);
  println!("{}", grid2);
}

fn create_grid(input: i32, width: usize, height: usize) -> CharGrid {
  let mut char_grid =CharGrid::new(width, height, '.');
  
  for y in 0..height {
    for x in 0..width {
      let result=x*x + 3*x + 2*x*y + y + y*y + input as usize;
      if result.count_ones()%2== 1 {
        char_grid.set(x,y,'*');
      };
    }
  }
  
  char_grid
}