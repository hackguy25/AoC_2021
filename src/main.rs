#![allow(non_snake_case)]
#![allow(dead_code)]

use std::fs;

fn day_01() {
    // read and parse data
    let data = fs::read_to_string("inputs/day_01.in").expect("aaa");
    let mut data: Vec<&str> = data.split("\n").collect();
    let _ = data.pop();
    let data: Vec<u32> = data
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    // count number of times the depth increases
    let mut count_single: u32 = 0;
    for i in 1..data.len() {
        if data[i] > data[i - 1] {
            count_single += 1;
        }
    }

    // count number of times the three measurement sum increases
    let mut count_triple: u32 = 0;
    let mut prev_sum = data[0] + data[1] + data[2];
    for i in 1..(data.len() - 2) {
        let sum = data[i] + data[i + 1] + data[i + 2];
        if sum > prev_sum {
            count_triple += 1;
        }
        prev_sum = sum;
    }

    println!("{:?}, {:?}", count_single, count_triple);
}

fn day_02() {
    // read and parse data
    let data = fs::read_to_string("inputs/day_02.in").expect("aaa");
    let data = data
        .lines()
        .filter_map(|x| {
            let y: Vec<_> = x.split(" ").collect();
            if y.len() < 2 {
                // line doesn't contain 2 distinct elements
                return None;
            }
            let n = match y[1].parse::<i32>() {
                Ok(x) => x,
                Err(_) => return None,
            };
            let c = match y[0].chars().nth(0) {
                Some(x) => x,
                None => return None,
            };
            Some((c, n))
        })
        .collect::<Vec<_>>();

    // compute submarine's path
    let mut depth = 0;
    let mut horiz = 0;
    for (c, n) in &data {
        match c {
            'f' => horiz += n,
            'd' => depth += n,
            'u' => depth -= n,
            _ => (),
        }
    }
    let first = depth * horiz;

    // compute path with aim
    let mut depth = 0;
    let mut horiz = 0;
    let mut aim = 0;
    for (c, n) in &data {
        match c {
            'f' => {
                horiz += n;
                depth += aim * n;
            }
            'd' => aim += n,
            'u' => aim -= n,
            _ => (),
        }
    }

    // print result
    println!("({}, {})", first, depth * horiz);
}

fn main() {
    // day_01();
    day_02();
}
