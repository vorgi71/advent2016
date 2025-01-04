fn main() {
  let start = std::time::Instant::now();
    let result_a = dragon_curve("1");
    println!("Result a: {}", result_a);
    assert_eq!(result_a, "100");
    let result_b = dragon_curve("0");
    println!("Result b:{}", result_b);
    assert_eq!(result_b, "001");
    let result_d = dragon_curve("111100001010");
    println!("Result d:{}", result_d);
    assert_eq!(result_d, "1111000010100101011110000");
    let result_e = check_sum("110010110100".to_string());
    assert_eq!(result_e, "100");

    let input1 = "10000";
    let result1 = fill_disk(input1, 20);
    println!("Result 1: {}", result1);
    assert_eq!(result1, "01100");

    let result2 = fill_disk("10001110011110000", 272);
    println!("Result 2: {}", result2);

    let result3 = fill_disk("10001110011110000", 35_651_584);
    println!("Result 3: {}", result3);
  let end_time = start.elapsed();
  println!("Elapsed time: {:?}", end_time);
}

fn fill_disk(input: &str, disk_size: usize) -> String {
    let mut fill_string = input.to_string();

    while fill_string.len() < disk_size {
        fill_string = dragon_curve(&fill_string);
    }

    fill_string = fill_string[..disk_size].to_string();

    check_sum(fill_string)
}

fn check_sum(input: String) -> String {
    let mut input_chars: Vec<char> = input.chars().collect();
    let mut new_sum: Vec<char> = Vec::new();
    loop {
        for index in (0..input_chars.len()).step_by(2) {
            let new_char = if input_chars[index] == input_chars[index + 1] {
                '1'
            } else {
                '0'
            };
            new_sum.push(new_char);
        }
        if new_sum.len() % 2 == 1 {
            break;
        }
        input_chars = new_sum.clone();
        new_sum.clear();
    }

    new_sum.iter().collect()
}

fn dragon_curve(a: &str) -> String {
    let b = a
        .chars()
        .rev()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect::<String>();
    format!("{a}0{b}")
}
