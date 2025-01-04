mod utils;

use md5::{Digest, Md5};
use regex::Regex;
use std::collections::HashMap;
use std::iter::Map;

fn main() {
    let input1 = "abc";

    let result1 = find_64_hash(input1, hash);
    println!("day 14 1: {}", result1);

    let input2 = "qzyelonm";
    let result2 = find_64_hash(input2, hash);
    println!("day 14 2: {}", result2);

    let result3 = find_64_hash(input1, hash_2016);
    println!("day 14 3: {}", result3);
    
    let result4 = find_64_hash(input2, hash_2016);
    println!("day 14 4: {}", result4);
}

fn find_64_hash<F>(salt: &str, hash_func: F) -> u64
where
    F: Fn(&str, u64, &mut HashMap<u64, String>) -> String,
{
    let mut index: u64 = 0;
    let mut found_keys: Vec<(String, u64)> = vec![];

    let mut memory: HashMap<u64, String> = HashMap::new();

    while found_keys.len() < 64 {
        let hash_code = hash_func(salt, index, &mut memory);

        let found_triplets = find_triplets(&hash_code);
        if !found_triplets.is_empty() {
            println!("{hash_code} {:?}", found_triplets);
            let triplet = found_triplets.first().unwrap();
            let search_string = triplet.clone() + &triplet[1..3];
            for next_index in index + 1..index + 1000 {
                let next_hash = hash_func(salt, next_index, &mut memory);
                if next_hash.contains(search_string.as_str()) {
                    println!("found: {triplet:?} {search_string:?} {next_hash:?}");
                    found_keys.push((hash_code.clone(), index));
                    break;
                }
            }
        }

        index += 1;
    }

    let mut key_index: u64 = 0;
    found_keys.iter().for_each(|(hash, index)| {
        println!("{}:{} {}", key_index, index, hash);
        key_index += 1;
    });

    let last_key = found_keys.last().unwrap().clone();
    last_key.1
}

fn find_triplets(hex_string: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let mut queue: Vec<char> = Vec::new();

    for c in hex_string.chars() {
        queue.push(c);
        if (queue.len() > 3) {
            queue.remove(0);
        }
        let first = queue[0];
        let mut allsame = true;
        if queue.len() < 3 {
            allsame = false;
        } else {
            for index in 1..queue.len() {
                if queue[index] != first {
                    allsame = false;
                    break;
                }
            }
        }
        if allsame {
            result.push(format!("{}{}{}", queue[0], queue[1], queue[2]));
        }
    }

    result
}

fn hash(salt: &str, index: u64, memory: &mut HashMap<u64, String>) -> String {
    if let Some(result) = memory.get(&index) {
        return result.clone();
    }
    let mut hasher = Md5::new();
    hasher.update(format!("{salt}{}", index));
    let result = hasher.finalize();

    let result = format!("{:x}", result);
    memory.insert(index, result.clone());

    result
}

fn hash_2016(salt: &str, index: u64, memory: &mut HashMap<u64, String>) -> String {
    if let Some(result) = memory.get(&index) {
        return result.clone();
    }
    let mut input = format!("{salt}{}", index);
    let mut output: String;
    for i in 0..=2016 {
        let mut hasher = Md5::new();
        hasher.update(input);
        output = format!("{:x}", hasher.finalize());

        input = output;
    }
    memory.insert(index, input.parse().unwrap());

    input.to_string()
}

// aaa  babbb
