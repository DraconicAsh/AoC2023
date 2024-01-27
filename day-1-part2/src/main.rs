use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, usize};

const NUMS: &str = "zero one two three four five six seven eight nine";
const NUMS_INDEXED: [Num; 20] = Num::new_array([
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2",
    "3", "4", "5", "6", "7", "8", "9",
]);

fn main() {
    let args: Vec<String> = env::args().collect();

    let f = File::open(&args[1]).unwrap();
    let mut input = BufReader::with_capacity(22528, f);
    let mut total = 0_usize;

    let mut line = String::new();
    loop {
        line.clear();
        let len = input.read_line(&mut line).unwrap();
        print!("{line}");
        if len == 0 {
            break;
        }
        total += parse_line(&mut line);
    }

    println!("Answer: {total}");
}

#[derive(Debug, Clone, Copy)]
struct Num {
    text: &'static str,
    val: usize,
}

impl Num {
    const fn new_array(nums: [&'static str; 20]) -> [Self; 20] {
        let mut res = [Self::blank(); 20];
        let mut i = 0;
        while i < 20 {
            res[i] = Self {
                text: nums[i],
                val: (i % 10),
            };
            i += 1;
        }
        res
    }

    const fn blank() -> Self {
        Self { text: "", val: 0 }
    }
}

struct PlacedNum<'a>(&'a Num, usize);

fn parse(line: &mut String) -> Vec<usize> {
    let mut res = Vec::new();
    let mut tmp = Vec::new();
    for (i, n) in NUMS_INDEXED.iter().enumerate() {
        if let Some(p) = line.find(n.text) {
            tmp.push(PlacedNum(&NUMS_INDEXED[i], p));
        }
        if let Some(p) = line.rfind(n.text) {
            tmp.push(PlacedNum(&NUMS_INDEXED[i], p));
        }
    }
    tmp.sort_by(|a, b| a.1.cmp(&b.1));
    for n in tmp {
        res.push(n.0.val);
    }
    res
}

//fn parse(line: &mut String) -> Vec<u32> {
//    let mut res = Vec::new();
//    let mut tex_num = String::new();
//    for c in line.chars() {
//        if c.is_numeric() {
//            res.push(c.to_digit(10).unwrap());
//            tex_num.clear();
//            continue;
//        }
//
//        tex_num.push(c);
//        if let Some(n) = NUMS_INDEXED.iter().position(|&x| x == &tex_num) {
//            res.push(n as u32);
//        } else if NUMS.contains(&tex_num) {
//            continue;
//        }
//
//        tex_num = match tex_num.as_str() {
//            "nini" => "ni".to_string(),
//            "eei" => "ei".to_string(),
//            "fon" => "on".to_string(),
//            _ => c.to_string(),
//        };
//    }
//    println!("{:?}", res);
//    res
//}

fn parse_line(line: &mut String) -> usize {
    let mut digits = parse(line);
    let first = digits[0];
    let second = digits.pop().unwrap();
    let res = (first * 10) + second;
    println!("{res}\n");
    res
}
