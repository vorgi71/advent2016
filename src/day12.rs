mod utils;
use utils::utils::read_lines;

fn main() {
  let input1 = read_lines("data/day12_1.txt");
  let result1 = interpret_lines(&input1);
  println!("result1:{:?}",result1);
  
  let input2 = read_lines("data/day12_2.txt");
  let result2 = interpret_lines(&input2);
  println!("result2:{:?}",result2);
}

fn interpret_lines(lines: &Vec<String>) -> (i32, i32, i32, i32) {
  let mut registers = (0, 0, 1, 0);

  let mut pc: isize = 0;

  while pc >= 0 && pc < lines.len() as isize {
    let line = &lines[pc as usize];
    let instruction = &line[0..3];
    match instruction {
      "cpy" => {
        let mut source = 0;
        let rest = &line[4..].split(" ").collect::<Vec<&str>>();
        match rest[0].parse::<i32>() {
          Ok(number) => {
            source = number;
          }
          Err(_) => {
            source = match rest[0] {
              "a" => registers.0,
              "b" => registers.1,
              "c" => registers.2,
              "d" => registers.3,
              _ => panic!("unknown argument: {:?}", rest),
            }
          }
        }
        match rest[1] {
          "a" => registers.0 = source,
          "b" => registers.1 = source,
          "c" => registers.2 = source,
          "d" => registers.3 = source,
          _ => panic!("unknown argument: {:?}", rest),
        }
      }
      "inc" => {
        let rest = &line[4..];
        match rest {
          "a" => {
            registers.0 += 1;
          }
          "b" => {
            registers.1 += 1;
          }
          "c" => {
            registers.2 += 1;
          }
          "d" => {
            registers.3 += 1;
          }
          _ => panic!("unknown argument: {:?}", line),
        }
      }
      "dec" => {
        let rest = &line[4..];
        match rest {
          "a" => {
            registers.0 -= 1;
          }
          "b" => {
            registers.1 -= 1;
          }
          "c" => {
            registers.2 -= 1;
          }
          "d" => {
            registers.3 -= 1;
          }
          _ => panic!("unknown argument: {:?}", line),
        }
      }
      "jnz" => {
        let rest = &line[4..].split(" ").collect::<Vec<&str>>();
        let not_zero = match rest[0] {
          "a" => registers.0 != 0,
          "b" => registers.1 != 0,
          "c" => registers.2 != 0,
          "d" => registers.3 != 0,
          number => number.parse::<i32>().unwrap()!=0,
        };
        if not_zero {
          let pc_value = rest[1].parse::<isize>().unwrap();
          pc += pc_value-1;
        }
      }
      _ => {
        panic!("unexpected line! {}", line);
      }
    }
    pc += 1;
  }

  registers
}
