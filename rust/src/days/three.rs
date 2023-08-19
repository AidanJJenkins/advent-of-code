use std::collections::HashMap;
use std::io::BufRead;

use crate::days::one::open_sesame;


pub fn ruck() -> Result<(), Box<dyn std::error::Error>> {
    let input = open_sesame("day_3_input");

    let mut alphabet_map: HashMap<String, usize> = HashMap::new();
    let alphabet = String::from("abcdefghijklmnopqrstuvwxyz");

    let mut pos = 0;
    let mut upper = 26;
    for letter in alphabet.chars() {
        pos += 1;
        upper += 1;
        alphabet_map.insert(letter.to_string(), pos);
        alphabet_map.insert(letter.to_uppercase().to_string(), upper);
    }

    let mut result: usize = 0;

    for i in input?.lines() {
        let i = i.unwrap();

        let l = find(i.to_string());
        if l != l.to_lowercase() {
            if let Some(value) = alphabet_map.get(&l) {
                result += value;
            }
        } else {
            if let Some(value) = alphabet_map.get(&l) {
                result += value;
            }
        }
    }

    println!("day 3a: {}", result);
    Ok(())
}

fn find(input: String) -> String {
    let midpoint = input.len() / 2;
    let first_half = &input[..midpoint];
    let second_half = &input[midpoint..];

    let mut s: String = String::new();

    for letter in first_half.chars() {
        if second_half.contains(letter) {
            s = letter.to_string()
        }
    }

    s
}

pub fn ruck_part_2() -> Result<(), Box<dyn std::error::Error>> {
    let input = open_sesame("day_3_input");
    let mut alphabet_map: HashMap<String, usize> = HashMap::new();

    let alphabet = String::from("abcdefghijklmnopqrstuvwxyz");

    let mut pos = 0;
    let mut upper = 26;
    for letter in alphabet.chars() {
        pos += 1;
        upper += 1;
        alphabet_map.insert(letter.to_string(), pos);
        alphabet_map.insert(letter.to_uppercase().to_string(), upper);
    }

    let mut badge: Vec<String> = Vec::new();

    let mut temp: Vec<String> = Vec::new();
    for line_res in input?.lines() {
        let line = line_res?;
        temp.push(line.to_string());
        if temp.len() == 3 {
            let one = temp.pop().unwrap();
            let two = temp.pop().unwrap();
            let three = temp.pop().unwrap();
            let res = new_find(&one, &two, &three);

            badge.push(res);
        } 
    }


    let mut points = 0;
    for i in badge {
        let point = alphabet_map.get(&i).unwrap();
        points = points + point;
    }
    println!("day 3b: {:?}", points);
    Ok(())
}

fn new_find(one: &str, two: &str, three: &str) -> String {
    let mut s = String::new();

    for letter in one.chars() {
        if two.contains(letter) && three.contains(letter) {
            s = letter.to_string()
        }
    }
    s
}
