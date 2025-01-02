fn main() {
  let map1= vec![vec!["HM","LM"],vec!["HG"],vec!["LG"],vec![]];

  let result1=solve_map(map1, 0, vec![vec![], vec![], vec![], vec![]]);
  println!("{:?}", result1);
}

fn solve_map<'a>(map: Vec<Vec<&'a str>>, level: i32, mut visited:Vec<Vec<Vec<&'a str>>>) -> i32 {
  visited.push(map.clone());

  println!("{:?}", map);

  let possible_moves=calculate_possible_moves(map,level, visited);
  
  println!("{:?}",possible_moves);

  0
}

fn calculate_possible_moves<'a>(map: Vec<Vec<&'a str>>, level: i32, visited: Vec<Vec<Vec<&'a str>>>) -> Vec<(i32, Vec<Vec<&'a str>>)> {
  let mut possible_moves = vec![];
  
  for next_level in (level-1..=level+1).step_by(2) {
    if next_level<0 || next_level>=map.len() as i32 {
      continue;
    }
    for d1_index in 0..=(map[level as usize].len()) {
      let d1= if d1_index<map[level as usize].len() {map[level as usize][d1_index] } else {""};
      for d2_index in d1_index..=(map[level as usize].len()) {
        let d2= if d2_index<map[level as usize].len() {map[level as usize][d2_index] } else {""};

        if(d1==d2) {
          continue;
        }

        println!("d1: {} d2: {}",d1,d2);

        let new_map=map.clone();
        let new_map=move_device(new_map.clone(), d1, d1_index, d2, d2_index, level, next_level);

        let check_map=check_map(&new_map);

        println!("new_map={:?} {}",new_map,check_map);
        if check_map {
          possible_moves.push((next_level,new_map));
        }
      }
    }
  }

  possible_moves
}

fn move_device<'a>(mut map: Vec<Vec<&'a str>>, d1:&'a str, d1_index:usize, d2:&'a str, mut d2_index:usize, level: i32, next_level:i32) -> Vec<Vec<&'a str>> {
  let from: &mut Vec<&str> =map.get_mut(level as usize).unwrap();
  if !d1.is_empty() {
    from.remove(d1_index);
    d2_index-=1;
  }

  if !d2.is_empty() {
    from.remove(d2_index);
  }

  let to=map.get_mut(next_level as usize).unwrap();

  if !d1.is_empty() {
    to.push(d1);
  }

  if !d2.is_empty() {
    to.push(d2);
  }

  to.sort();

  map
}

fn check_map(map: &Vec<Vec< &str>>) -> bool {
  for level in map.iter() {
    let chips=level.iter().find(|x| { x.ends_with("M")});
    let gens = level.iter().find(|x| { x.ends_with("G") });
    let danger=!gens.iter().all(|gen| {
      let chip_name=format!("{}{}",gen.chars().next().unwrap(),'M');
      if chips.is_some() {
        chips.unwrap().contains(&chip_name)
      } else {
        false
      }
    });
    let additional_chips=gens.iter().any(|gen| {
      let chip_name=format!("{}{}",gen.chars().next().unwrap(),'M');
      if chips.is_some() {
        !chips.unwrap().contains(&chip_name)
      } else { false }
    });
    if danger && additional_chips {
      return false;
    }
  }

  true
}