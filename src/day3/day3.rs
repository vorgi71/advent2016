fn main() {
  print!("day3 ");

  let input1="    4   21  894
  419  794  987
  424  797  125
  651  305  558
  651  305  254
  651  305  558";

  let result1 = valid_triangles(input1);
  print!("{}\n", result1);

  let input2 = read_file("data/day3/day3.txt");
  let result2 = valid_triangles(&input2);
  print!("{}\n", result2);

  let result3 = valid_triangles(&input1);
  print!("{}\n", result3);

  let result4=valid_triangles_2(&input2);
  print!("{}\n", result4);
}

pub fn read_file(file: &str) -> String {
  std::fs::read_to_string(file).unwrap()
}

fn valid_triangles(input1: &str) -> i32 {
  let mut sum = 0;

  input1.split("\n").for_each(|line| {
    let s1 = line[0..5].trim().parse::<i32>().unwrap();
    let s2 = line[7..10].trim().parse::<i32>().unwrap();
    let s3 = line[12..15].trim().parse::<i32>().unwrap();

    if s1 + s2 > s3 && s2 + s3 > s1 && s3 + s1 > s2 {
      sum += 1;
    }
  });
  sum
}

fn valid_triangles_2(input1: &str) -> i32 {
  let mut sum = 0;

  let mut data:Vec<Vec<i32>>=Vec::new();

  input1.split("\n").for_each(|line| {
    let s1=line[0..5].trim().parse::<i32>().unwrap();
    let s2=line[7..10].trim().parse::<i32>().unwrap();
    let s3=line[12..15].trim().parse::<i32>().unwrap();

    data.push(vec![s1,s2,s3]);

  });

  for i in (0..data.len()).step_by(3) {
    for j in 0..3 {
      let s1 = data[i][j];
      let s2 = data[i + 1][j];
      let s3 = data[i + 2][j];

      if s1 + s2 > s3 && s2 + s3 > s1 && s3 + s1 > s2 {
        sum +=1;
      }
    }
  }


  sum
}