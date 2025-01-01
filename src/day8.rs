mod utils;

use std::str::FromStr;
use utils::utils::read_lines;
use crate::utils::utils::CharGrid;

fn main() {
  let input1=read_lines("data/day8_1.txt");
  println!("{:?}",input1);
  let result1=process_input(input1,7,3);
  println!("{:?}",result1);

  let input2=read_lines("data/day8_2.txt");
  let result2=process_input(input2,50,6);
  println!("{:?}",result2);

}

fn process_input(lines: Vec<String>,width:usize,height:usize) -> u32 {
  let mut sum:u32=0;

  let mut grid = CharGrid::new(width, height, '.');

  //ZJHRKCPLYJ

  for line in lines {
    if line.starts_with("rect") {
      let coord_string=&line[5..];
      let split=coord_string.split("x").collect::<Vec<&str>>();
      let x=usize::from_str(split[0]).unwrap();
      let y=usize::from_str(split[1]).unwrap();
      for yk in 0..y {
        for xk in 0..x {
          grid.set(xk,yk,'*');
        }
      }
      println!("{}",grid);
    } else if line.starts_with("rotate row") {
      let instruction_string=&line[13..];
      let split=instruction_string.split(" by ").collect::<Vec<&str>>();
      let line=usize::from_str(split[0]).unwrap();
      let amount=usize::from_str(split[1]).unwrap();

      println!("line {} by {}",line,amount);
      for _ in 0..amount {
        let helper=grid.get(grid.width-1,line);
        for index in (0..grid.width-1).rev() {
          grid.set(index+1,line,grid.get(index,line).unwrap());
        }
        grid.set(0,line,helper.unwrap());
      }

      println!("{}",grid);
    } else if line.starts_with("rotate column") {
      let instruction_string=&line[16..];
      let split=instruction_string.split(" by ").collect::<Vec<&str>>();
      let column=usize::from_str(split[0]).unwrap();
      let amount=usize::from_str(split[1]).unwrap();

      println!("column {} by {}",column,amount);
      for _ in 0..amount {
        let helper=grid.get(column,grid.height-1);
        for index in (0..grid.height-1).rev() {
          grid.set(column,index+1,grid.get(column,index).unwrap());
        }
        grid.set(column,0,helper.unwrap());
      }

      println!("{}",grid);
    }
  }

  for y in 0..grid.height {
    for x in 0..grid.width {
      if grid.get(x, y).unwrap_or('?')=='*' {
        sum+=1;
      }
    }
  }

  sum
}