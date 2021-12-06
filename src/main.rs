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
    println!("{}, {}", first, depth * horiz);
}

fn day_03() {
    // read data
    let data = fs::read_to_string("inputs/day_03.in").expect("aaa");

    // count appearances
    let mut counts = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for line in data.lines() {
        let mut chars = line.chars();
        for i in 0..12 {
            counts[i] += if chars.next() == Some('1') { 1 } else { -1 };
        }
    }

    // calculate epsilon and gamma
    let (mut epsilon, mut gamma) = (0, 0);
    for i in 0..12 {
        gamma *= 2;
        epsilon *= 2;
        if counts[i] >= 0 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    // determine oxygen generator rating
    let mut values = data.lines().collect::<Vec<_>>();
    for i in 0..12 {
        let mut count = 0;
        for value in &values {
            if value.chars().nth(i) == Some('1') {
                count += 1;
            } else {
                count -= 1;
            }
        }
        values = if count >= 0 {
            values
                .into_iter()
                .filter(|s| s.chars().nth(i) == Some('1'))
                .collect()
        } else {
            values
                .into_iter()
                .filter(|s| s.chars().nth(i) == Some('0'))
                .collect()
        };
    }
    let oxygen = i32::from_str_radix(values[0], 2).unwrap();

    // determine CO2 scrubber rating
    let mut values = data.lines().collect::<Vec<_>>();
    for i in 0..12 {
        let mut count = 0;
        for value in &values {
            if value.chars().nth(i) == Some('1') {
                count += 1;
            } else {
                count -= 1;
            }
        }
        values = if count < 0 {
            values
                .into_iter()
                .filter(|s| s.chars().nth(i) == Some('1'))
                .collect()
        } else {
            values
                .into_iter()
                .filter(|s| s.chars().nth(i) == Some('0'))
                .collect()
        };
        if values.len() == 1 {
            break;
        }
    }
    let co2 = i32::from_str_radix(values[0], 2).unwrap();

    // print result
    println!("{}, {}", gamma * epsilon, oxygen * co2);
}

fn day_04() {
    // read data
    let data = fs::read_to_string("inputs/day_04.in").expect("aaa");

    // parse inputs
    let mut blocks = data.split("\n\n");
    let drawn_nums = blocks
        .next()
        .unwrap()
        .split(",")
        .filter_map(|x| x.parse::<i32>().ok())
        .collect::<Vec<_>>();
    let mut boards = blocks
        .map(|board| {
            board
                .split("\n")
                .filter(|line| line.len() > 9)
                .map(|line| line.split(" "))
                .map(|line| {
                    line.filter_map(|num| num.parse::<i32>().ok())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|board| Some(board))
        .collect::<Vec<_>>();

    // transpose boards for easier column detection
    let mut boards_transposed = (&boards)
        .into_iter()
        .map(|board| {
            Some(
                (0..5)
                    .map(|i| {
                        board
                            .as_ref()
                            .unwrap()
                            .into_iter()
                            .map(|line| line[i])
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    // play the drawn numbers
    let (mut winning, mut last_drawn) = (0, 0);
    let mut nums = drawn_nums.into_iter();
    loop {
        let num = nums.next().unwrap();
        let mut break_out = false;
        for (b_num, board) in (&mut boards).into_iter().enumerate() {
            for line in board.as_mut().unwrap() {
                line.retain(|x| *x != num);
                if line.is_empty() {
                    winning = b_num;
                    last_drawn = num;
                    break_out = true;
                }
            }
        }
        for (b_num, board) in (&mut boards_transposed).into_iter().enumerate() {
            for line in board.as_mut().unwrap() {
                line.retain(|x| *x != num);
                if line.is_empty() {
                    winning = b_num;
                    last_drawn = num;
                    break_out = true;
                }
            }
        }
        if break_out {
            break;
        }
    }

    // calculate the score
    let winning_board = &(boards[winning]);
    let winning_sum: i32 = winning_board
        .as_ref()
        .unwrap()
        .into_iter()
        .map(|line| line.into_iter().sum::<i32>())
        .sum();
    let last_drawn_winning = last_drawn;

    // play out the rest of the boards
    boards[winning] = None;
    boards_transposed[winning] = None;
    let (losing, last_drawn);
    'outer2: loop {
        let num = nums.next().unwrap();
        let mut to_delete = vec![];
        for (b_num, board) in (&mut boards).into_iter().enumerate() {
            match board.as_mut() {
                None => {
                    continue;
                }
                Some(lines) => {
                    for line in lines {
                        line.retain(|x| *x != num);
                        if line.is_empty() {
                            to_delete.push((true, b_num));
                        }
                    }
                }
            }
        }
        for (b_num, board) in (&mut boards_transposed).into_iter().enumerate() {
            match board.as_mut() {
                None => {
                    continue;
                }
                Some(lines) => {
                    for line in lines {
                        line.retain(|x| *x != num);
                        if line.is_empty() {
                            to_delete.push((false, b_num));
                        }
                    }
                }
            }
        }
        for (s, i) in to_delete {
            if s && (&boards).into_iter().filter(|x| **x != None).count() < 2 {
                losing = i;
                last_drawn = num;
                break 'outer2;
            } else if !s
                && (&boards_transposed)
                    .into_iter()
                    .filter(|x| **x != None)
                    .count()
                    < 2
            {
                losing = i;
                last_drawn = num;
                break 'outer2;
            } else {
                boards[i] = None;
                boards_transposed[i] = None;
            }
        }
    }

    // calculate the score and print result
    let losing_board = &(boards[losing]);
    let losing_sum: i32 = losing_board
        .as_ref()
        .unwrap()
        .into_iter()
        .map(|line| line.into_iter().sum::<i32>())
        .sum();
    println!(
        "{}, {}",
        winning_sum * last_drawn_winning,
        losing_sum * last_drawn
    );
}

fn day_05() {
    // read the data
    let data = fs::read_to_string("inputs/day_05.in").expect("aaa");
    let lines = data
        .lines()
        .map(|line| {
            let mut data = line.split(" -> ").map(|n| n.split(","));
            let mut first = data.next().unwrap();
            let mut second = data.next().unwrap();
            (
                first.next().unwrap().parse::<usize>().unwrap(),
                first.next().unwrap().parse::<usize>().unwrap(),
                second.next().unwrap().parse::<usize>().unwrap(),
                second.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    // consider only vertical and horizontal lines
    let general = (&lines).into_iter().filter(|(a, b, c, d)| a == c || b == d);

    // plot general lines on a field
    let mut field = vec![vec![0; 1000]; 1000];
    for (a, b, c, d) in general {
        if a == c {
            let (bot, top) = if b < d { (b, d) } else { (d, b) };
            for i in *bot..(top + 1) {
                field[*a][i] += 1;
            }
        } else {
            let (bot, top) = if a < c { (a, c) } else { (c, a) };
            for i in *bot..(top + 1) {
                field[i][*b] += 1;
            }
        }
    }

    // count number of cells with at least two lines going through them
    let general_overlap = field
        .into_iter()
        .map(|line| line.into_iter().filter(|x| *x >= 2).count())
        .sum::<usize>();

    // plot all lines on a field
    let mut field = vec![vec![0; 1000]; 1000];
    for (a, b, c, d) in (&lines).into_iter() {
        if a == c {
            let (bot, top) = if b < d { (b, d) } else { (d, b) };
            for i in *bot..(top + 1) {
                field[*a][i] += 1;
            }
        } else if b == d {
            let (bot, top) = if a < c { (a, c) } else { (c, a) };
            for i in *bot..(top + 1) {
                field[i][*b] += 1;
            }
        } else {
            let (dx, dy) = (2 * (c > a) as i32 - 1, 2 * (d > b) as i32 - 1);
            let (bot, top) = if a < c { (a, c) } else { (c, a) };
            let mut y = if a < c { *b } else { *d };
            for i in *bot..(top + 1) {
                field[i][y] += 1;
                if dx * dy > 0 {
                    y += 1;
                } else {
                    y -= 1;
                }
            }
        }
    }

    // count number of cells with at least two lines going through them
    let full_overlap = field
        .into_iter()
        .map(|line| line.into_iter().filter(|x| *x >= 2).count())
        .sum::<usize>();

    // print the result
    println!("{}, {}", general_overlap, full_overlap);
}

fn day_06() {
    // read and parse data
    let data = fs::read_to_string("inputs/day_06.in").expect("aaa");
    let state = data.lines().next().unwrap().split(",");
    let state = state.map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let mut state = (0..9).map(|n| state.iter().filter(|x| **x == n).count()).collect::<Vec<_>>();
    println!("{:?}", state);

    // simulate 256 days
    let mut after80 = 0;
    for iter in 0..256 {
        state[7] += state[0];
        let breeders = state.remove(0);
        state.push(breeders);
        if iter == 79 {
            after80 = state.iter().sum();
        }
    }

    // print the result
    println!("{}, {}", after80, state.iter().sum::<usize>());
}

fn main() {
    // day_01();
    // day_02();
    // day_03();
    // day_04();
    // day_05();
    day_06();
}
