#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::fs;
use std::vec::Vec;

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
    let mut state = (0..9)
        .map(|n| state.iter().filter(|x| **x == n).count())
        .collect::<Vec<_>>();

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

fn day_07() {
    // read and parse data
    let data = fs::read_to_string("inputs/day_07.in").expect("aaa");
    let state = data
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    // find min, max position
    let (min, max) = (
        *state.iter().min().unwrap(),
        state.iter().max().unwrap() + 1,
    );

    // determine naive cost for each position
    let mut costs = vec![];
    for i in min..max {
        costs.push(state.iter().map(|x| (x - i).abs()).sum::<i32>());
    }
    let smallest_naive = costs.iter().min().unwrap();

    // determine proper cost for each position
    let mut costs = vec![];
    for i in min..max {
        costs.push(
            state
                .iter()
                .map(|x| (x - i).abs())
                .map(|x| x * (x + 1) / 2)
                .sum::<i32>(),
        );
    }
    let smallest_proper = costs.iter().min().unwrap();

    // print the result
    println!("{}, {}", smallest_naive, smallest_proper);
}

fn day_08() {
    // read data
    let data = fs::read_to_string("inputs/day_08.in").expect("aaa");

    // count digits with unique signal counts
    let unique = data
        .lines()
        .flat_map(|x| x.split(" | ").nth(1).unwrap().split(" "))
        .map(|x| x.chars().count())
        .filter(|&x| x == 2 || x == 3 || x == 4 || x == 7)
        .count();

    // calculate sum of displayed numbers
    let mut acc = 0;
    for line in data.lines() {
        let mut line_it = line.split(" | ");

        // parse reference digits as sets of signal lines
        let mut refs = line_it
            .next()
            .unwrap()
            .split(" ")
            .map(|x| x.chars().collect::<HashSet<_>>())
            .collect::<Vec<_>>();

        // by sorting by length we get [1,7,4,x,x,x,y,y,y,8] where
        // x are 2, 3 and 5, and y are 0, 6 and 9, but mixed up
        refs.sort_unstable_by(|a, b| a.len().cmp(&b.len()));
        assert_eq!(refs.len(), 10);

        // sort out 2, 3 and 5
        match (
            refs[2].intersection(&refs[3]).count(),
            refs[2].intersection(&refs[4]).count(),
        ) {
            (3, 2) => {
                refs.swap(3, 4);
            }
            (3, 3) => {
                refs.swap(3, 5);
            }
            _ => (),
        };
        if refs[0].intersection(&refs[4]).count() == 1 {
            refs.swap(4, 5);
        }

        // sort out 0, 6 and 9
        match (
            refs[2].intersection(&refs[6]).count(),
            refs[2].intersection(&refs[7]).count(),
        ) {
            (4, 3) => {
                refs.swap(6, 8);
            }
            (3, 4) => {
                refs.swap(7, 8);
            }
            _ => (),
        };
        if refs[0].intersection(&refs[6]).count() == 1 {
            refs.swap(6, 7);
        }

        // now we have [1,7,4,2,3,5,0,6,9,8]
        let dig_list: [i32; 10] = [1, 7, 4, 2, 3, 5, 0, 6, 9, 8];

        // parse displayed digits
        let disp = line_it
            .next()
            .unwrap()
            .split(" ")
            .map(|x| x.chars().collect::<HashSet<_>>())
            .collect::<Vec<_>>();

        // calculate displayed value
        let mut num = 0;
        for digit in disp.iter() {
            num *= 10;
            let pos = refs.iter().position(|x| *x == *digit).unwrap();
            num += dig_list[pos];
        }

        // add displayed value to accumulator
        acc += num;
    }

    // print the result
    println!("{}, {}", unique, acc);
}

fn day_09() {
    // read and parse data
    let data = fs::read_to_string("inputs/day_09.in").expect("aaa");
    let mut data = data
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // find risk points
    let mut risk_points = vec![];
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if (i == 0 || data[i - 1][j] > data[i][j])
                && (i == data.len() - 1 || data[i + 1][j] > data[i][j])
                && (j == 0 || data[i][j - 1] > data[i][j])
                && (j == data[i].len() - 1 || data[i][j + 1] > data[i][j])
            {
                risk_points.push((i, j));
            }
        }
    }

    // sum risk levels
    let risk_sum = risk_points
        .iter()
        .map(|(i, j)| data[*i][*j] + 1)
        .sum::<u32>();

    // find basin sizes
    let mut basin_sizes = vec![];
    fn recurse(data: &mut Vec<Vec<u32>>, i: i32, j: i32) -> u32 {
        if i < 0
            || i >= data.len() as i32
            || j < 0
            || j >= data[i as usize].len() as i32
            || data[i as usize][j as usize] == 9
        {
            0
        } else {
            data[i as usize][j as usize] = 9;
            recurse(data, i - 1, j)
                + recurse(data, i + 1, j)
                + recurse(data, i, j - 1)
                + recurse(data, i, j + 1)
                + 1
        }
    }
    for (i, j) in &risk_points {
        basin_sizes.push(recurse(&mut data, *i as i32, *j as i32));
    }

    // find and sum three largest basin sizes
    basin_sizes.sort_unstable_by(|a, b| b.cmp(a));
    let basin_sizes = basin_sizes.iter().take(3).product::<u32>();

    // print the result
    println!("{}, {}", risk_sum, basin_sizes);
}

fn day_10() {
    // read data
    let data = fs::read_to_string("inputs/day_10.in").expect("aaa");

    // find corrupted lines
    let mut syntax_score = 0;
    let mut incomplete_lines = vec![];
    let opening = HashSet::from(['(', '[', '{', '<']);
    let closing = HashSet::from([')', ']', '}', '>']);
    let prices = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let matching = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    'outer: for l in data.lines() {
        let chars = l.chars();
        let mut open = vec![];
        for c in chars {
            if opening.contains(&c) {
                open.push(c);
            } else if closing.contains(&c) {
                match open.pop() {
                    Some(d) => {
                        if d != matching[&c] {
                            syntax_score += prices[&c];
                            continue 'outer;
                        }
                    }
                    None => {
                        syntax_score += prices[&c];
                        continue 'outer;
                    }
                }
            }
        }
        incomplete_lines.push(open);
    }

    // fix incomplete lines
    let mut scores = vec![];
    let prices = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    for mut l in incomplete_lines {
        let mut line_score = 0_u64;
        loop {
            match l.pop() {
                Some(c) => {
                    line_score *= 5;
                    line_score += prices[&c];
                }
                None => {
                    break;
                }
            }
        }
        scores.push(line_score);
    }

    // find the middle score
    scores.sort();
    let autocomplete_score = scores[scores.len() / 2];

    // print the result
    println!("{}, {}", syntax_score, autocomplete_score);
}

fn main() {
    // day_01();
    // day_02();
    // day_03();
    // day_04();
    // day_05();
    // day_06();
    // day_07();
    // day_08();
    // day_09();
    day_10();
}
