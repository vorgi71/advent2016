fn main() {
  let result1 = elephant_party(5);
  println!("result1 = {}", result1);
  let result2 = elephant_party(3005290);
  println!("result2 = {}", result2);

  let result3 = elephant_party_2(5);
  println!("result3 = {}", result3);

  let result4 = elephant_party_2(3005290);
  println!("result4 = {}", result4);
}

fn get_next_elf(table:&[i32],index:usize) -> usize {
  let mut current_index = index;
  current_index = (current_index + 1) % table.len();
  while table[current_index] == 0 && current_index != index {
    current_index = (current_index + 1) % table.len();
  }

  current_index
}

fn elephant_party(number_of_elves: i32) -> usize {
  let mut table = vec![1;number_of_elves as usize];

  let mut current_index = 0;

  loop {
    let next = get_next_elf(&table, current_index);
    if next==current_index {
      break;
    }
    table[current_index] += table[next];
    table[next] = 0;
    current_index=get_next_elf(&table,current_index); // elf has been robbed, so skip it
  }

  current_index
}

fn elephant_party_2(number_of_elves: usize) -> usize {
  let mut table:Vec<(i32,usize)> = vec![];

  for i in 1..=number_of_elves {
    table.push((1,i));
  }

  let mut current_index = 0;
  loop {
    if table.len()==1 {break;}

    let next_index = get_opposing_elf(&table, current_index);

    if(table.len()<1000) {
      println!("{} steals from {}", table[current_index].1, table[next_index].1);
    }
    table[current_index].0 += table[next_index].0;
    table.remove(next_index);

    if current_index<=next_index {
      current_index = (current_index + 1) % table.len();
    } else {
      current_index %= table.len();
    }
    
  }

  table[0].1
}

fn get_opposing_elf(table: &Vec<(i32, usize)>, index: usize) -> usize {
  (index + table.len()/2) % table.len()
}