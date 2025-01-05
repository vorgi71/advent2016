fn main() {
  let result1 = create_map(".^^.^.^^^^",10);
  println!("result 1: {:?}", result1);
  
  let result2 = create_map("^.^^^..^^...^.^..^^^^^.....^...^^^..^^^^.^^.^^^^^^^^.^^.^^^^...^^...^^^^.^.^..^^..^..^.^^.^.^.......",40);
  println!("result 2: {:?}", result2);

  let result3 = create_map("^.^^^..^^...^.^..^^^^^.....^...^^^..^^^^.^^.^^^^^^^^.^^.^^^^...^^...^^^^.^.^..^^..^..^.^^.^.^.......",40000);
  println!("result 3: {:?}", result3);
}

fn create_map(start: &str, length: i32) -> i32 {
  let mut result: Vec<String> = vec![start.to_string()];
  let mut chars = start.chars().collect::<Vec<char>>();
  let mut new_line:Vec<char>=vec![];
  
  for i in 1..length { 
    for line_index in 0..chars.len() {
      let c0 = if line_index>0 {chars.get(line_index - 1).unwrap()} else {&'.'};
      let c1 = chars.get(line_index).unwrap_or(&'.');
      let c2 = if line_index<chars.len()-1 {chars.get(line_index + 1).unwrap() } else {&'.'};

      if *c0 == '^' && *c1 == '^' && *c2 == '.' {
        new_line.push('^');
      } else if *c0 == '.' && *c1 == '^' && *c2 == '^' {
        new_line.push('^');
      } else if *c0 == '^' && *c1 == '.' && *c2 == '.' {
        new_line.push('^');
      } else if *c0 == '.' && *c1 == '.' && *c2 == '^' {
        new_line.push('^');
      } else {
        new_line.push('.');
      }
    }
    
    let new_string=new_line.iter().collect::<String>();
    chars=new_string.clone().chars().collect::<Vec<char>>();
    result.push(new_string);
    new_line.clear();
  }
  
  result.iter().for_each(|line| {
    println!("{}", line);
  });
  
  result.iter().fold(0,|acc, line| {
    acc+line.chars().fold(0,|acc,c| {
      acc+if c == '.' {1} else {0}
    })
  })
}