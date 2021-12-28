#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};
use std::fs;
use std::io::{self, Write};
use std::iter::{once, repeat};
use std::rc::Rc;
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

fn day_11() {
    // read data
    let data = fs::read_to_string("inputs/day_11.in").expect("aaa");

    // prepare grid
    let mut grid = vec![vec![0; 12]];
    for l in data.lines() {
        let mut line = vec![0];
        line.extend(l.chars().filter_map(|c| c.to_digit(10)).map(|c| c as i32));
        line.push(0);
        grid.push(line)
    }
    grid.push(vec![0; 12]);

    // prepare neighbor calculation
    let neighbors = |i: usize, j: usize| {
        [
            (i - 1, j - 1),
            (i - 1, j),
            (i - 1, j + 1),
            (i, j - 1),
            (i, j + 1),
            (i + 1, j - 1),
            (i + 1, j),
            (i + 1, j + 1),
        ]
    };

    // simulate steps
    let mut flashes = 0;
    let mut flashes_100 = 0;
    let mut sync_step = 0;
    for i in 0..1000 {
        // increment all energies by 1
        for i in 1..11 {
            for j in 1..11 {
                grid[i][j] += 1;
            }
        }

        // loop until all flashes are taken care of
        loop {
            let mut updates = 0;
            for i in 1..11 {
                for j in 1..11 {
                    if grid[i][j] > 9 {
                        for (k, l) in neighbors(i, j) {
                            grid[k as usize][l as usize] += 1;
                        }
                        grid[i][j] = i32::MIN;
                        updates += 1;
                    }
                }
            }
            if updates == 0 {
                break;
            };
        }

        // count flashes and reset counters
        let mut curr_flashes = 0;
        for i in 1..11 {
            for j in 1..11 {
                if grid[i][j] < 0 {
                    grid[i][j] = 0;
                    curr_flashes += 1;
                }
            }
        }
        flashes += curr_flashes;

        // note number of flashes after 100 steps
        if i == 99 {
            flashes_100 = flashes;
        }

        // check if the octopi have synchronized
        if curr_flashes == 100 {
            sync_step = i + 1;
            break;
        }
    }

    // print the result
    println!("{}, {}", flashes_100, sync_step);
}

fn day_12() {
    // read data
    let data = fs::read_to_string("inputs/day_12.in").expect("aaa");

    // parse possible paths
    let mut paths: HashMap<String, HashSet<String>> = HashMap::new();
    for line in data.lines() {
        let mut line = line.split("-");
        let a = line.next().unwrap().to_owned();
        let b = line.next().unwrap().to_owned();
        if let Some(s) = paths.get_mut(&a) {
            s.insert(b.clone());
        } else {
            paths.insert(a.clone(), HashSet::from([b.clone()]));
        }
        if let Some(s) = paths.get_mut(&b) {
            s.insert(a.clone());
        } else {
            paths.insert(b.clone(), HashSet::from([a.clone()]));
        }
    }
    paths.get_mut(&String::from("end")).unwrap().clear();

    // prepare the first recursive function
    fn rec_1(
        pos: &String,
        path: &mut HashSet<String>,
        paths: &HashMap<String, HashSet<String>>,
    ) -> u64 {
        let possible = paths.get(pos).unwrap();
        if possible.len() == 0 {
            (pos == "end") as u64
        } else {
            let mut total_paths = 0;
            if pos.chars().next().unwrap().is_lowercase() {
                path.insert(pos.clone());
            }
            for p in possible.iter() {
                if !path.contains(p) {
                    total_paths += rec_1(&p, path, paths);
                }
            }
            path.remove(pos);
            total_paths
        }
    }

    // prepare the second recursive function
    fn rec_2(
        pos: &String,
        path: &mut HashSet<String>,
        paths: &HashMap<String, HashSet<String>>,
        repeated: &mut Option<String>,
    ) -> u64 {
        let possible = paths.get(pos).unwrap();
        if possible.len() == 0 {
            (pos == "end") as u64
        } else {
            let mut total_paths = 0;
            for p in possible.iter() {
                if !path.contains(p) {
                    if p.chars().next().unwrap().is_lowercase() {
                        path.insert(p.clone());
                    }
                    total_paths += rec_2(&p, path, paths, repeated);
                    path.remove(p);
                } else if *repeated == None && p != "start" {
                    *repeated = Some(pos.clone());
                    total_paths += rec_2(&p, path, paths, repeated);
                    *repeated = None;
                }
            }
            total_paths
        }
    }

    // count all paths
    let all_paths = rec_1(&String::from("start"), &mut HashSet::new(), &paths);
    let all_paths_repeated = rec_2(
        &String::from("start"),
        &mut HashSet::from(["start".to_owned()]),
        &paths,
        &mut None,
    );

    // print the result
    println!("{}, {}", all_paths, all_paths_repeated);
}

fn day_13() {
    // read and parse data
    let data = fs::read_to_string("inputs/day_13.in").expect("aaa");
    let (mut dots, mut lines, mut folds) = (HashSet::new(), data.lines(), vec![]);
    loop {
        match lines.next().unwrap() {
            "" => {
                break;
            }
            ll => {
                let mut coords = ll.split(",");
                dots.insert((
                    coords.next().unwrap().parse::<i32>().unwrap(),
                    coords.next().unwrap().parse::<i32>().unwrap(),
                ));
            }
        }
    }
    for l in lines {
        let mut parts = l.split("=");
        folds.push((
            parts.next().unwrap().chars().nth(11).unwrap(),
            parts.next().unwrap().parse::<i32>().unwrap(),
        ));
    }

    // prepare fold procedure
    let make_fold = |dots: &HashSet<_>, fold: (char, i32)| {
        let mut new_dots = HashSet::new();
        if fold.0 == 'x' {
            for (x, y) in dots {
                new_dots.insert((i32::max(fold.1 - x, x - fold.1) - 1, *y));
            }
        } else {
            for (x, y) in dots {
                new_dots.insert((*x, i32::max(fold.1 - y, y - fold.1) - 1));
            }
        }
        new_dots
    };

    // make and print the first fold
    let mut folds_iter = folds.iter();
    let mut new_dots = make_fold(&dots, *folds_iter.next().unwrap());
    let after_first = new_dots.len();
    println!("{}", after_first);

    // finish folding
    for fold in folds.iter() {
        dots = new_dots;
        new_dots = make_fold(&dots, *fold);
    }

    // assemble and print the result
    let mut result = new_dots.into_iter().collect::<Vec<_>>();
    result.sort_by(
        |(ax, ay), (bx, by)| {
            if ay == by {
                ax.cmp(bx)
            } else {
                ay.cmp(by)
            }
        },
    );
    let (mut x, mut y) = (-1, 0);
    for (a, b) in result {
        if b > y {
            y = b;
            x = -1;
            println!("");
        };
        while a > x + 1 {
            print!(".");
            x += 1;
        }
        print!("#");
        x += 1;
    }
    println!("");
}

fn day_14() {
    // read and parse data
    let data = fs::read_to_string("inputs/day_14.in").expect("aaa");
    let mut lines = data.lines();
    let template = lines.next().unwrap();
    lines.next();
    let mut insertion_rules = HashMap::new();
    for line in lines {
        let mut parts = line.split(" -> ");
        insertion_rules.insert(
            parts.next().unwrap(),
            parts.next().unwrap().chars().next().unwrap(),
        );
    }

    // convert template into a collection of pairs
    let mut pairs = HashMap::<String, usize>::new();
    let mut template_chars = template.chars();
    let mut prev_char = template_chars.next().unwrap();
    for next_char in template_chars {
        if let Some(count) = pairs.get_mut(&String::from_iter([prev_char, next_char])) {
            *count += 1;
        } else {
            pairs.insert(String::from_iter([prev_char, next_char]), 1);
        }
        prev_char = next_char;
    }

    // apply substitution process 40 times
    let mut pairs_10 = HashMap::new();
    for i in 0..40 {
        let mut new_pairs = HashMap::<String, usize>::new();
        for (key, count) in pairs.iter() {
            if let Some(c) = insertion_rules.get(key.as_str()) {
                for new_key in [
                    String::from_iter([key.chars().nth(0).unwrap(), *c]),
                    String::from_iter([*c, key.chars().nth(1).unwrap()]),
                ] {
                    if let Some(c) = new_pairs.get_mut(&new_key) {
                        *c += count;
                    } else {
                        new_pairs.insert(new_key, *count);
                    }
                }
            } else {
                if let Some(c) = new_pairs.get_mut(key) {
                    *c += count;
                } else {
                    new_pairs.insert(key.clone(), *count);
                }
            }
        }
        pairs = new_pairs;
        if i == 9 {
            pairs_10 = pairs.clone();
        }
    }

    // find difference in quantities of most and least common elements
    let mut elements = HashMap::new();
    for (key, count) in pairs_10 {
        for pos in [0, 1] {
            if let Some(c) = elements.get_mut(&key.chars().nth(pos).unwrap()) {
                *c += count;
            } else {
                elements.insert(key.chars().nth(pos).unwrap(), count);
            }
        }
    }
    // each element is counted exactly twice, except for the first and last
    elements = elements
        .into_iter()
        .map(|(k, v)| (k, (v + 1) / 2))
        .collect();
    let q_diff_10 = elements.values().max().unwrap() - elements.values().min().unwrap();

    // same but for the state after 40 steps
    let mut elements = HashMap::new();
    for (key, count) in pairs {
        for pos in [0, 1] {
            if let Some(c) = elements.get_mut(&key.chars().nth(pos).unwrap()) {
                *c += count;
            } else {
                elements.insert(key.chars().nth(pos).unwrap(), count);
            }
        }
    }
    elements = elements
        .into_iter()
        .map(|(k, v)| (k, (v + 1) / 2))
        .collect();
    let q_diff_40 = elements.values().max().unwrap() - elements.values().min().unwrap();

    // print result
    println!("{}, {}", q_diff_10, q_diff_40);
}

fn day_15() {
    // read and parse data
    let data = fs::read_to_string("inputs/day_15.in").expect("aaa");
    let risks = data
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // dynamic programming approach:
    // start at (0, 0)
    // every time a tile is updated, if its risk changed, add its neighbors into queue
    // to update a tile check its neighbors, select the one with smallest risk and add its own risk
    // repeat until the queue is empty

    // prepare auxiliary items
    struct TileState {
        risk: u8,
        total_risk: u32,
        selected_neighbor: (i8, i8),
    }
    let (height, width) = (risks.len() as i32 + 2, risks[0].len() as i32 + 2);
    let border_tile = || TileState {
        risk: 255,
        total_risk: u32::MAX,
        selected_neighbor: (0, 0),
    };

    // prepare field
    let mut field = Vec::new();
    field.reserve((height * width) as usize);
    field.extend((0..width).map(|_| border_tile()));
    for l in &risks {
        field.push(border_tile());
        for r in l {
            field.push(TileState {
                risk: *r,
                total_risk: u32::MAX,
                selected_neighbor: (0, 0),
            })
        }
        field.push(border_tile());
    }
    field.extend((0..width).map(|_| border_tile()));

    // adjust starting point
    field[width as usize + 1].risk = 0;
    field[width as usize + 1].total_risk = 0;

    // prepare first neighbors
    let mut queue = Vec::new();
    queue.push((2_i32, 1_i32));
    queue.push((1, 2));

    // loop through candidates
    while let Some((x, y)) = queue.pop() {
        // ignore border cells
        if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
            continue;
        }

        // check all neighbors
        let mut notify = false;
        for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            // let current = &mut field[(y * width + x) as usize];
            // let neighbor = &field[((y+dy) * width + (x+dx)) as usize];
            let new_risk = field[(y * width + x) as usize].risk as u64
                + field[((y + dy) * width + (x + dx)) as usize].total_risk as u64;
            if new_risk < field[(y * width + x) as usize].total_risk as u64 {
                let current = &mut field[(y * width + x) as usize];
                current.total_risk = new_risk as u32;
                current.selected_neighbor = (dx as i8, dy as i8);
                notify = true;
            }
        }
        if notify {
            for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                queue.push((x + dx, y + dy));
            }
        }
    }

    // extract risk
    print!("{}", field[((height - 1) * width - 2) as usize].total_risk);

    // same procedure, but for way larger field
    let (height, width) = (risks.len() as i32 * 5 + 2, risks[0].len() as i32 * 5 + 2);
    let mut field = Vec::new();
    field.reserve((height * width) as usize);
    field.extend((0..width).map(|_| border_tile()));
    for dy in 0..5 {
        for l in &risks {
            field.push(border_tile());
            for dx in 0..5 {
                for r in l {
                    let mut rr = r + dx + dy;
                    while rr > 9 {
                        rr -= 9;
                    }
                    field.push(TileState {
                        risk: rr,
                        total_risk: u32::MAX,
                        selected_neighbor: (0, 0),
                    })
                }
            }
            field.push(border_tile());
        }
    }
    field.extend((0..width).map(|_| border_tile()));

    field[width as usize + 1].risk = 0;
    field[width as usize + 1].total_risk = 0;

    let mut queue = Vec::new();
    queue.push((2_i32, 1_i32));
    queue.push((1, 2));

    while let Some((x, y)) = queue.pop() {
        if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
            continue;
        }

        let mut notify = false;
        for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let new_risk = field[(y * width + x) as usize].risk as u64
                + field[((y + dy) * width + (x + dx)) as usize].total_risk as u64;
            if new_risk < field[(y * width + x) as usize].total_risk as u64 {
                let current = &mut field[(y * width + x) as usize];
                current.total_risk = new_risk as u32;
                current.selected_neighbor = (dx as i8, dy as i8);
                notify = true;
            }
        }
        if notify {
            for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                queue.push((x + dx, y + dy));
            }
        }
    }

    let risk_2 = field[((height - 1) * width - 2) as usize].total_risk;
    println!(", {}", risk_2);
}

fn day_16() {
    // read and prepare bits
    let data = fs::read_to_string("inputs/day_16.in").expect("aaa");
    let packet = data
        .trim()
        .chars()
        .flat_map(|c| (0..4).map(move |n| ((1 << (3 - n) & c.to_digit(16).unwrap()) > 0) as u8))
        .collect::<Vec<_>>();
    // println!("  {:?}", packet);

    // recursive function summing up the version numbers of all subpackets
    fn sum_versions<I: Iterator<Item = u8>>(data: &mut I) -> Option<u32> {
        let version = data.next()? * 4 + data.next()? * 2 + data.next()?;
        let type_ID = data.next()? * 4 + data.next()? * 2 + data.next()?;
        if type_ID == 4 {
            // literal value
            // println!("literal: {} {}", version, type_ID);
            // let mut count = 1;
            while data.next()? == 1 {
                data.nth(3);
                // count += 1;
            }
            data.nth(3);
            // println!("  {:?}", count);
            Some(version as u32)
        } else {
            // operator
            let mut acc = version as u32;
            if data.next()? == 0 {
                // println!("operator, len: {} {}", version, type_ID);
                // next 15 bits represent the total bit length of subpackets
                let mut bit_len = 0;
                for _ in 0..15 {
                    bit_len *= 2;
                    bit_len += data.next()? as usize;
                }
                let subpackets_v = data.by_ref().take(bit_len).collect::<Vec<_>>();
                // println!("  {:?}", subpackets_v);
                let mut subpackets = subpackets_v.into_iter();

                while let Some(n) = sum_versions(&mut subpackets) {
                    acc += n;
                }
            } else {
                // next 11 bits represent number of subpackets
                // println!("operator, num: {} {}", version, type_ID);
                let mut bit_len = 0;
                for _ in 0..11 {
                    bit_len *= 2;
                    bit_len += data.next()? as usize;
                }
                // println!("  {:?}", bit_len);
                for _ in 0..bit_len {
                    acc += sum_versions(data)?;
                }
            }
            Some(acc)
        }
    }

    // sum up versions
    print!("{}", sum_versions(&mut packet.clone().into_iter()).unwrap());

    // recursive function summing up the version numbers of all subpackets
    fn evaluate<I: Iterator<Item = u8>>(data: &mut I) -> Option<u64> {
        let _ = data.next()? * 4 + data.next()? * 2 + data.next()?;
        let type_ID = data.next()? * 4 + data.next()? * 2 + data.next()?;
        if type_ID == 4 {
            // literal value
            // println!("literal: {}", type_ID);
            let mut acc = 0;
            while data.next()? == 1 {
                for _ in 0..4 {
                    acc *= 2;
                    acc += data.next()? as u64;
                }
            }
            for _ in 0..4 {
                acc *= 2;
                acc += data.next()? as u64;
            }
            // println!("  {:?}", acc);
            Some(acc)
        } else {
            // operator, extract subpackets
            let mut acc = vec![];
            if data.next()? == 0 {
                // println!("operator, len: {}", type_ID);
                // next 15 bits represent the total bit length of subpackets
                let mut bit_len = 0;
                for _ in 0..15 {
                    bit_len *= 2;
                    bit_len += data.next()? as usize;
                }
                let subpackets_v = data.by_ref().take(bit_len).collect::<Vec<_>>();
                // println!("  {:?}", subpackets_v);
                let mut subpackets = subpackets_v.into_iter();

                while let Some(n) = evaluate(&mut subpackets) {
                    acc.push(n);
                }
            } else {
                // next 11 bits represent number of subpackets
                // println!("operator, num: {}", type_ID);
                let mut bit_len = 0;
                for _ in 0..11 {
                    bit_len *= 2;
                    bit_len += data.next()? as usize;
                }
                // println!("  {:?}", bit_len);
                for _ in 0..bit_len {
                    acc.push(evaluate(data)?);
                }
            }
            // println!("  {:?}", acc);
            match type_ID {
                0 => Some(acc.iter().sum()),                    // sum packet
                1 => Some(acc.iter().product()),                // product packet
                2 => Some(*acc.iter().min()?),                  // minimum packet
                3 => Some(*acc.iter().max()?),                  // maximum packet
                5 => Some((acc.get(0)? > acc.get(1)?) as u64),  // greater than packet
                6 => Some((acc.get(0)? < acc.get(1)?) as u64),  // less than packet
                7 => Some((acc.get(0)? == acc.get(1)?) as u64), // equal to packet
                _ => None,
            }
        }
    }

    // evaluate expression
    println!(", {}", evaluate(&mut packet.clone().into_iter()).unwrap());
}

fn day_17() {
    // read the data
    let data = fs::read_to_string("inputs/day_17.in").expect("aaa");
    let mut data = data
        .split("x=")
        .nth(1)
        .unwrap()
        .trim()
        .split(", y=")
        .flat_map(|x| x.split(".."))
        .map(|x| x.parse::<i32>().unwrap());
    let area = [
        data.next().unwrap(),
        data.next().unwrap(),
        data.next().unwrap(),
        data.next().unwrap(),
    ];

    // simple 2D point
    struct Vec2 {
        x: i32,
        y: i32,
    }
    impl std::ops::AddAssign<&Vec2> for Vec2 {
        fn add_assign(&mut self, other: &Self) {
            self.x += other.x;
            self.y += other.y;
        }
    }

    // function that checks if probe lands inside the target area
    fn lands(x_vel: i32, y_vel: i32, area: &[i32; 4]) -> bool {
        let mut pos = Vec2 { x: 0, y: 0 };
        let mut vel = Vec2 { x: x_vel, y: y_vel };

        loop {
            if pos.x >= area[0] && pos.x <= area[1] && pos.y >= area[2] && pos.y <= area[3] {
                // probe lies inside the area
                return true;
            }
            if pos.x > area[1] || vel.y <= 0 && pos.y < area[2] {
                // probe overshot the area
                return false;
            }
            pos += &vel;
            if vel.x > 0 {
                vel.x -= 1;
            }
            vel.y -= 1;
        }
    }

    // find minimum viable x velocity
    let mut min_x_vel = 1;
    loop {
        let max_x = (min_x_vel + 1) * min_x_vel / 2;
        if max_x >= area[0] {
            break;
        }
        min_x_vel += 1;
    }

    // find viable trajectory with the largest y value
    let mut best_max_y = i32::MIN;
    let mut target_vel = Vec2 { x: 0, y: 0 };
    let mut num_possible = 0;
    for x_vel in min_x_vel..(area[1] + 1) {
        for y_vel in area[2]..1000 {
            if lands(x_vel, y_vel, &area) {
                let max_y = (y_vel + 1) * y_vel / 2;
                num_possible += 1;
                if max_y > best_max_y {
                    best_max_y = max_y;
                    target_vel.x = x_vel;
                    target_vel.y = y_vel;
                }
            }
        }
    }

    // display the result
    println!("{}, {}", best_max_y, num_possible);
}

fn day_18() {
    // read the data
    let data = fs::read_to_string("inputs/day_18.in").expect("aaa");

    // define a pair type
    #[derive(Clone)]
    enum Element {
        Value(i32),
        Pair(Rc<(Element, Element)>),
    }
    impl Element {
        fn value(self) -> i32 {
            if let Value(n) = self {
                n.clone()
            } else {
                panic!("Not a value")
            }
        }
        fn pair(&mut self) -> (Element, Element) {
            if let Pair(rc) = self {
                Rc::make_mut(rc).clone()
            } else {
                panic!("Not a pair")
            }
        }
    }
    impl std::fmt::Debug for Element {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Value(n) => n.fmt(f),
                Pair(p) => {
                    let (l, r) = &**p;
                    f.debug_list().entry(l).entry(r).finish()
                }
            }
        }
    }
    use Element::*;

    // recursive number parsing function
    fn parse_num(source: &mut std::str::Chars) -> Element {
        if let Some(n) = source.next().unwrap().to_digit(10) {
            Value(n as i32)
        } else {
            let l = parse_num(source);
            source.next().unwrap();
            let r = parse_num(source);
            source.next().unwrap();
            Pair(Rc::new((l, r)))
        }
    }

    // parse the data
    let mut numbers = data
        .lines()
        .map(|l| parse_num(&mut l.chars()))
        .collect::<Vec<_>>();
    numbers.reverse(); // allows popping from the front of vec

    // number reducing function
    fn reduce(num: &mut Element) {
        fn add_to_leftmost(num: &mut Element, val: i32) -> Element {
            match num {
                Value(v) => Value(*v + val),
                Pair(p) => {
                    let (l, r) = Rc::make_mut(p);
                    Pair(Rc::new((add_to_leftmost(l, val), r.clone())))
                }
            }
        }
        fn add_to_rightmost(num: &mut Element, val: i32) -> Element {
            match num {
                Value(v) => Value(*v + val),
                Pair(p) => {
                    let (l, r) = Rc::make_mut(p);
                    Pair(Rc::new((l.clone(), add_to_rightmost(r, val))))
                }
            }
        }

        // explode the leftmost pair of depth 5
        fn explode(num: &mut Element, depth: i32) -> Option<(i32, i32)> {
            if depth == 4 {
                if let Pair(p) = num {
                    let (l, r) = Rc::make_mut(p).clone();
                    *num = Value(0);
                    Some((l.value(), r.value()))
                } else {
                    None
                }
            } else {
                if let Pair(p) = num {
                    let (l, r) = Rc::make_mut(p);
                    if let Some((ll, rr)) = explode(l, depth + 1) {
                        *num = Pair(Rc::new((l.clone(), add_to_leftmost(r, rr))));
                        Some((ll, 0))
                    } else if let Some((ll, rr)) = explode(r, depth + 1) {
                        *num = Pair(Rc::new((add_to_rightmost(l, ll), r.clone())));
                        Some((0, rr))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }

        // split the leftmost regular number over 9
        fn split(num: &mut Element) -> bool {
            match num {
                Value(n) => {
                    if *n > 9 {
                        *num = Pair(Rc::new((Value(*n / 2), Value((*n + 1) / 2))));
                        true
                    } else {
                        false
                    }
                }
                Pair(p) => {
                    let (l, r) = Rc::get_mut(p).unwrap();
                    split(l) || split(r)
                }
            }
        }

        loop {
            // try exploding, else try splitting, else return
            if let Some(_) = explode(num, 0) {
                continue;
            } else if split(num) {
                continue;
            } else {
                break;
            }
        }
    }

    // function that calculates magnitude of a number
    fn magnitude(num: &Element) -> i32 {
        match num {
            Value(n) => *n,
            Pair(p) => {
                let (l, r) = &**p;
                3 * magnitude(l) + 2 * magnitude(r)
            }
        }
    }

    // reduce all numbers
    let mut num_iter = numbers.iter();
    let mut acc = num_iter.next().unwrap().clone();
    for next in num_iter {
        acc = Pair(Rc::new((acc, next.clone())));
        reduce(&mut acc);
    }

    // find largest magnitude of sum of 2 distinct numbers
    let mut max_magnitude = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }
            let mut acc = Pair(Rc::new((numbers[i].clone(), numbers[j].clone())));
            reduce(&mut acc);
            let curr_magnitude = magnitude(&acc);
            max_magnitude = std::cmp::max(max_magnitude, curr_magnitude);
        }
    }

    // display the result
    println!("{}, {}", magnitude(&acc), max_magnitude);
}

fn day_19() {
    // read the data
    let data = fs::read_to_string("inputs/day_19.in").expect("aaa");
    let scanners = data.split("\n\n");
    let scanners = scanners
        .map(|b| {
            b.lines()
                .skip(1)
                .map(|l| {
                    let point: [i32; 3] = l
                        .split(",")
                        .map(|n| n.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                        .as_slice()
                        .try_into()
                        .unwrap();
                    point
                })
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<_>>();

    // function that rotates a point into one of 24 rotations
    fn rotate(rot: usize, [x, y, z]: [i32; 3]) -> [i32; 3] {
        let [x, y, z] = match rot / 4 {
            0 => [x, y, z],
            1 => [y, -x, z],
            2 => [-x, -y, z],
            3 => [-y, x, z],
            4 => [z, y, -x],
            5 => [-z, y, x],
            _ => panic!("Not a valid rotation"),
        };
        match rot % 4 {
            0 => [x, y, z],
            1 => [x, z, -y],
            2 => [x, -y, -z],
            3 => [x, -z, y],
            _ => panic!("Not a valid rotation"),
        }
    }
    // table of inverse rotations
    // rotate(inv_rot[r], rotate(r, point)) == point
    let inv_rot = [
        0, 3, 2, 1, 12, 19, 6, 21, 8, 9, 10, 11, 4, 23, 14, 17, 20, 15, 18, 5, 16, 7, 22, 13,
    ];

    // sanity check
    for r in 0..24 {
        assert_eq!(rotate(inv_rot[r], rotate(r, [0, 1, 2])), [0, 1, 2])
    }

    // set of connections between scanners, (from, to, rotation, delta)
    let mut connections = vec![];

    // check all pairs of scanners for overlaps
    for from in 0..(scanners.len() - 1) {
        'outer: for to in (from + 1)..scanners.len() {
            // check all rotations and point overlaps
            let reference = &scanners[from];
            let compared = &scanners[to];
            for rot in 0..24 {
                let compared = compared.iter().map(|x| rotate(rot, *x)).collect::<Vec<_>>();
                for refe in reference {
                    'inner: for comp in &compared {
                        let dx = refe[0] - comp[0];
                        let dy = refe[1] - comp[1];
                        let dz = refe[2] - comp[2];
                        let compared = compared
                            .iter()
                            .map(|[x, y, z]| [x + dx, y + dy, z + dz])
                            .filter(|[x, y, z]| {
                                *x >= -1000
                                    && *x <= 1000
                                    && *y >= -1000
                                    && *y <= 1000
                                    && *z >= -1000
                                    && *z <= 1000
                            })
                            .collect::<Vec<_>>();
                        if compared.len() >= 12 {
                            for point in compared {
                                if !reference.contains(&point) {
                                    // extraneous point => false overlap
                                    continue 'inner;
                                }
                            }
                            // 12 or more points, all matching both beacons
                            // => found overlap
                            connections.push((from, to, rot, [dx, dy, dz]));
                            continue 'outer;
                        }
                    }
                }
            }
        }
    }

    // find positions and rotations of scanners
    let mut positions = HashMap::new();
    positions.insert(0_usize, (vec![], [0, 0, 0]));

    while connections.len() > 0 {
        // unused connections remain
        let mut remaining_connections = vec![];
        for (from, to, rot, [mut dx, mut dy, mut dz]) in connections {
            if positions.contains_key(&from) {
                if positions.contains_key(&to) {
                    // both beacons are already located
                    continue;
                } else {
                    // only source is located
                    // find position of the source
                    let (mut rots, [x, y, z]) = positions[&from].clone();
                    // recalculate delta based on rotations
                    rots.reverse();
                    for r in &rots {
                        let delta = rotate(*r, [dx, dy, dz]);
                        dx = delta[0]; // at this point destructuring assignments
                        dy = delta[1]; // were not stable yet
                        dz = delta[2];
                    }
                    rots.reverse();
                    rots.push(rot);
                    // insert newly calculated position
                    positions.insert(to, (rots, [x + dx, y + dy, z + dz]));
                }
            } else {
                if positions.contains_key(&to) {
                    // only the destination is located => switch their places
                    let inv = inv_rot[rot];
                    remaining_connections.push((to, from, inv, rotate(inv, [-dx, -dy, -dz])));
                } else {
                    // both beacons are not located yet => try later
                    remaining_connections.push((from, to, rot, [dx, dy, dz]));
                }
            }
        }
        connections = remaining_connections;
    }

    // compile beacons
    let mut beacons = HashSet::new();
    let ppositions = positions
        .iter()
        .map(|(n, (v, a))| (*n, (v.clone(), a.clone())))
        .collect::<Vec<_>>();
    for (scanner, (mut rots, [x, y, z])) in ppositions {
        rots.reverse();
        let new_beacons = scanners[scanner].iter().map(|[dx, dy, dz]| {
            let (mut dx, mut dy, mut dz) = (*dx, *dy, *dz);
            for r in &rots {
                let delta = rotate(*r, [dx, dy, dz]);
                dx = delta[0]; // at this point destructuring assignments
                dy = delta[1]; // were not stable yet
                dz = delta[2];
            }
            [x + dx, y + dy, z + dz]
        });
        beacons.extend(new_beacons);
    }

    // find largest manhattan distance
    let mut acc = 0;
    for i in 0..(positions.len() - 1) {
        for j in (i + 1)..positions.len() {
            let curr = positions[&i]
                .1
                .iter()
                .zip(positions[&j].1.iter())
                .map(|(x, y)| (x - y).abs())
                .sum::<i32>();
            acc = std::cmp::max(acc, curr);
        }
    }

    // display the result
    println!("{}, {}", beacons.len(), acc);
}

fn day_20() {
    // read the data
    let data = fs::read_to_string("inputs/day_20.in").expect("aaa");
    let mut data_chunks = data.split("\n\n");
    let algorithm = data_chunks
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<_>>();
    let initial_image = data_chunks
        .next()
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // function that applies the image enhancement algorithm
    // takes an image and the background, returns new image and background
    fn enhance(
        (image, bg): (Vec<Vec<bool>>, bool),
        algorithm: &Vec<bool>,
    ) -> (Vec<Vec<bool>>, bool) {
        // pad the image properly
        let width = image[0].len();
        let image = image.into_iter().map(|l| {
            let mut ret = vec![bg, bg];
            ret.extend(l);
            ret.extend([bg, bg]);
            ret
        });
        let image = image
            .chain(once(vec![bg; width + 4]))
            .chain(once(vec![bg; width + 4]));

        // iterate over the lines and collect
        let (mut pprev, mut prev) = (vec![bg; width + 4], vec![bg; width + 4]);
        let new_image = image
            .map(|line| {
                // iterator over triplets of elements
                let mut line_iter = pprev.iter().zip(prev.iter().zip(line.iter()));
                // prepare first two triplets
                let temp = line_iter.next().unwrap();
                let (pp, (cp, np)) = temp;
                let temp = line_iter.next().unwrap();
                let (pc, (cc, nc)) = temp;
                let mut p = (*pp, *cp, *np);
                let mut c = (*pc, *cc, *nc);
                // iterate over the elements
                let ret = line_iter
                    .map(|(pn, (cn, nn))| {
                        let n = (*pn, *cn, *nn);
                        let idx = [p.0, c.0, n.0, p.1, c.1, n.1, p.2, c.2, n.2]
                            .iter()
                            .fold(0, |a, x| 2 * a + *x as usize);
                        p = c;
                        c = n;
                        algorithm[idx]
                    })
                    .collect::<Vec<_>>();
                pprev = prev.clone();
                prev = line;
                ret
            })
            .collect::<Vec<_>>();

        // return new image and enhanced background
        (new_image, algorithm[511 * bg as usize])
    }

    // apply enhancement algorithm twice
    let (image, bg) = enhance((initial_image.clone(), false), &algorithm);
    let (mut image, mut bg) = enhance((image, bg), &algorithm);
    assert_eq!(bg, false);

    // count number of light pixels
    let light = image
        .iter()
        .map(|l| l.iter().filter(|x| **x).count())
        .sum::<usize>();

    // apply enhancement 48 more times
    for _ in 0..48 {
        let ret = enhance((image, bg), &algorithm);
        image = ret.0;
        bg = ret.1;
    }

    // count number of light pixels
    let light_full = image
        .iter()
        .map(|l| l.iter().filter(|x| **x).count())
        .sum::<usize>();

    // display the result
    println!("{}, {}", light, light_full);
}

fn day_21() {
    // read the data
    let data = fs::read_to_string("inputs/day_21.in").expect("aaa");
    let mut lines = data
        .lines()
        .map(|l| l.split(": ").nth(1).unwrap().parse::<i32>());
    let (p1_start, p2_start) = (
        lines.next().unwrap().unwrap(),
        lines.next().unwrap().unwrap(),
    );

    // the mythical deterministic die
    struct Die {
        pub rolls: i32,
    }
    impl Die {
        fn roll(&mut self) -> i32 {
            let ret = (self.rolls % 100) + 1;
            self.rolls += 1;
            ret
        }
    }
    let mut die = Die { rolls: 0 };

    // play the game
    let (mut p1_pos, mut p2_pos) = (p1_start - 1, p2_start - 1);
    let (mut p1_score, mut p2_score) = (0, 0);
    loop {
        // player 1
        let roll = die.roll() + die.roll() + die.roll();
        p1_pos += roll;
        p1_pos %= 10;
        p1_score += p1_pos + 1;
        if p1_score >= 1000 {
            break;
        }

        // player 2
        let roll = die.roll() + die.roll() + die.roll();
        p2_pos += roll;
        p2_pos %= 10;
        p2_score += p2_pos + 1;
        if p2_score >= 1000 {
            break;
        }
    }

    // calculate first result
    let val1 = std::cmp::min(p1_score, p2_score) * die.rolls;

    // possible universes after 3 rolls:
    // 3 4 5  4 5 6  5 6 7
    // 4 5 6  5 6 7  6 7 8
    // 5 6 7  6 7 8  7 8 9

    // the only difference is total sum:
    // (3 => 1 universe)
    // (4 => 3 universes)
    // (5 => 6 universes)
    // (6 => 7 universes)
    // (7 => 6 universes)
    // (8 => 3 universes)
    // (9 => 1 universe)
    let univ_count = [0_u64, 0, 0, 1, 3, 6, 7, 6, 3, 1];

    // state: (p1_pos, p1_score, p2_pos, p2_score, num_universes, player)
    let mut states = vec![(p1_start - 1, 0, p2_start - 1, 0, 1_u64, true)];
    let (mut p1_wins, mut p2_wins) = (0_u64, 0_u64);
    while let Some((p1_pos, p1_score, p2_pos, p2_score, num_universes, p1)) = states.pop() {
        if p1 {
            for i in 3..10 {
                let p1_pos = (p1_pos + i as i32) % 10;
                let p1_score = p1_score + p1_pos + 1;
                let num_universes = num_universes * univ_count[i];
                if p1_score >= 21 {
                    p1_wins += num_universes;
                } else {
                    states.push((p1_pos, p1_score, p2_pos, p2_score, num_universes, false));
                }
            }
        } else {
            for i in 3..10 {
                let p2_pos = (p2_pos + i as i32) % 10;
                let p2_score = p2_score + p2_pos + 1;
                let num_universes = num_universes * univ_count[i];
                if p2_score >= 21 {
                    p2_wins += num_universes;
                } else {
                    states.push((p1_pos, p1_score, p2_pos, p2_score, num_universes, true));
                }
            }
        }
    }

    // display the result
    println!("{}, {}", val1, std::cmp::max(p1_wins, p2_wins));
}

fn day_22() {
    // read and parse the data
    let data = fs::read_to_string("inputs/day_22.in").expect("aaa");
    let steps = data
        .lines()
        .map(|l| {
            let mut line = l.split(" ");
            let state = line.next().unwrap() == "on";
            let mut line = line.next().unwrap().trim().split(",");
            let mut lims = line.next().unwrap().split("=").nth(1).unwrap().split("..");
            let (xmin, xmax) = (lims.next().unwrap(), lims.next().unwrap());
            let (xmin, xmax) = (xmin.parse::<i32>().unwrap(), xmax.parse::<i32>().unwrap());
            let mut lims = line.next().unwrap().split("=").nth(1).unwrap().split("..");
            let (ymin, ymax) = (lims.next().unwrap(), lims.next().unwrap());
            let (ymin, ymax) = (ymin.parse::<i32>().unwrap(), ymax.parse::<i32>().unwrap());
            let mut lims = line.next().unwrap().split("=").nth(1).unwrap().split("..");
            let (zmin, zmax) = (lims.next().unwrap(), lims.next().unwrap());
            let (zmin, zmax) = (zmin.parse::<i32>().unwrap(), zmax.parse::<i32>().unwrap());
            (xmin, xmax, ymin, ymax, zmin, zmax, state)
        })
        .collect::<Vec<_>>();

    // prepare the small cuboid
    let mut small_cuboid = vec![false; 101 * 101 * 101];

    // turn on/off all cubes in the small cuboid
    for (xmin, xmax, ymin, ymax, zmin, zmax, state) in &steps {
        if *xmin > 50 || *xmax < -50 || *ymin > 50 || *ymax < -50 || *zmin > 50 || *zmax < -50 {
            // given cuboid doesn't overlap with the small cuboid
            continue;
        }
        let (xmin, xmax) = (max(-50, *xmin), min(50, *xmax));
        let (ymin, ymax) = (max(-50, *ymin), min(50, *ymax));
        let (zmin, zmax) = (max(-50, *zmin), min(50, *zmax));
        for x in xmin..=xmax {
            for y in ymin..=ymax {
                for z in zmin..=zmax {
                    small_cuboid
                        [10201 * (z + 50) as usize + 101 * (y + 50) as usize + (x + 50) as usize] =
                        *state;
                }
            }
        }
    }

    // count number of cubes turned on
    let small_lit = small_cuboid.iter().filter(|x| **x).count();

    // a set of range trees (kinda)
    struct Range1D {
        splits: Vec<i32>,
        values: Vec<bool>,
    }
    impl Range1D {
        fn construct(ranges: &Vec<(i32, i32, bool)>) -> Range1D {
            // determine all possible split points
            let mut splits = vec![];
            for (min, max, _) in ranges {
                splits.push(*min);
                splits.push(max + 1); // inclusive upper bound is given
            }
            splits.sort_unstable();
            splits.dedup();

            // apply steps onto ranges
            let mut values = if splits.len() > 0 {
                vec![false; splits.len() - 1]
            } else {
                vec![]
            };
            for (min, max, val) in ranges {
                let (min, max) = (*min, max + 1);
                for idx in
                    (splits.binary_search(&min).unwrap())..(splits.binary_search(&max).unwrap())
                {
                    values[idx] = *val;
                }
            }
            Range1D { splits, values }
        }
        // sum up ranges where cubes are lit up
        fn count(&self) -> u64 {
            if self.splits.len() > 0 {
                let mut prev = self.splits[0];
                let mut acc = 0;
                for i in 1..self.splits.len() {
                    if self.values[i - 1] {
                        acc += self.splits[i] - prev;
                    }
                    prev = self.splits[i];
                }
                acc as u64
            } else {
                0
            }
        }
    }

    struct Range2D {
        splits: Vec<i32>,
        values: Vec<Range1D>,
    }
    impl Range2D {
        fn construct(ranges: &Vec<(i32, i32, i32, i32, bool)>) -> Range2D {
            // determine all possible split points
            let mut splits = vec![];
            for (min, max, _, _, _) in ranges {
                splits.push(*min);
                splits.push(max + 1);
            }
            splits.sort_unstable();
            splits.dedup();

            // apply steps onto 1D ranges
            let mut values = if splits.len() > 0 {
                vec![vec![]; splits.len() - 1]
            } else {
                vec![]
            };
            for (min, max, mmin, mmax, val) in ranges {
                let (min, max) = (*min, max + 1);
                for idx in
                    (splits.binary_search(&min).unwrap())..(splits.binary_search(&max).unwrap())
                {
                    values[idx].push((*mmin, *mmax, *val));
                }
            }
            let values = values.into_iter().map(|v| Range1D::construct(&v)).collect();
            Range2D { splits, values }
        }

        // sum up counts of 1D ranges times range widths
        fn count(&self) -> u64 {
            if self.splits.len() > 0 {
                let mut prev = self.splits[0];
                let mut acc = 0;
                for i in 1..self.splits.len() {
                    acc += (self.splits[i] - prev) as u64 * self.values[i - 1].count();
                    prev = self.splits[i];
                }
                acc as u64
            } else {
                0
            }
        }
    }

    struct Range3D {
        splits: Vec<i32>,
        values: Vec<Range2D>,
    }
    impl Range3D {
        fn construct(ranges: &Vec<(i32, i32, i32, i32, i32, i32, bool)>) -> Range3D {
            // determine all possible split points
            let mut splits = vec![];
            for (min, max, _, _, _, _, _) in ranges {
                splits.push(*min);
                splits.push(max + 1);
            }
            splits.sort_unstable();
            splits.dedup();

            // apply steps onto 1D ranges
            let mut values = if splits.len() > 0 {
                vec![vec![]; splits.len() - 1]
            } else {
                vec![]
            };
            for (min, max, mmin, mmax, mmmin, mmmax, val) in ranges {
                let (min, max) = (*min, max + 1);
                for idx in
                    (splits.binary_search(&min).unwrap())..(splits.binary_search(&max).unwrap())
                {
                    values[idx].push((*mmin, *mmax, *mmmin, *mmmax, *val));
                }
            }
            let values = values.into_iter().map(|v| Range2D::construct(&v)).collect();
            Range3D { splits, values }
        }

        // sum up counts of 1D ranges times range widths
        fn count(&self) -> u64 {
            if self.splits.len() > 0 {
                let mut prev = self.splits[0];
                let mut acc = 0;
                for i in 1..self.splits.len() {
                    acc += (self.splits[i] - prev) as u64 * self.values[i - 1].count();
                    prev = self.splits[i];
                }
                acc as u64
            } else {
                0
            }
        }
    }

    // construct a range tree over the cuboids
    let tree = Range3D::construct(&steps);

    // display the result
    println!("{}, {}", small_lit, tree.count());
}

fn day_23() {
    // read and parse the data
    let data = fs::read_to_string("inputs/day_23.in").expect("aaa");
    let mut lines = data.lines();
    let mut rooms = vec![];
    rooms.push(
        lines
            .nth(2)
            .unwrap()
            .split("#")
            .filter(|x| *x != "")
            .map(|x| x.chars().next().unwrap())
            .collect::<Vec<_>>(),
    );
    for _ in 0..3 {
        rooms.push(
            lines
                .next()
                .unwrap()
                .trim()
                .split("#")
                .filter(|x| *x != "")
                .map(|x| x.chars().next().unwrap())
                .collect::<Vec<_>>(),
        );
    }

    // amphipod types
    #[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
    enum Amphipod {
        None,
        A,
        B,
        C,
        D,
    }
    impl Amphipod {
        fn from_char(c: char) -> Amphipod {
            match c {
                'A' => Amphipod::A,
                'B' => Amphipod::B,
                'C' => Amphipod::C,
                'D' => Amphipod::D,
                _ => Amphipod::None,
            }
        }
        fn cost(&self) -> usize {
            match self {
                Amphipod::A => 1,
                Amphipod::B => 10,
                Amphipod::C => 100,
                Amphipod::D => 1000,
                Amphipod::None => 0,
            }
        }
    }

    // an amphipod configuration
    // 0  1  x  2  x  3  x  4  x  5  6
    //       7     8     9    10
    //      11    12    13    14

    #[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
    struct State([Amphipod; 15]);
    impl State {
        fn swap(&self, a: usize, b: usize) -> State {
            let mut ret = self.clone();
            ret[a] = self[b];
            ret[b] = self[a];
            ret
        }
    }
    impl std::ops::Index<usize> for State {
        type Output = Amphipod;
        fn index(&self, i: usize) -> &Amphipod {
            &self.0[i]
        }
    }
    impl std::ops::IndexMut<usize> for State {
        fn index_mut(&mut self, i: usize) -> &mut Amphipod {
            &mut self.0[i]
        }
    }

    // starting configuration
    let mut start = [Amphipod::None; 15];
    for i in 0..4 {
        start[7 + i] = Amphipod::from_char(rooms[0][i]);
        start[11 + i] = Amphipod::from_char(rooms[3][i]);
    }
    let start = State(start);

    // all known configurations and lowest known energies to get to them from start
    let mut costs = HashMap::new();
    costs.insert(start, 0_usize);

    // queue of states to consider
    let mut queue = VecDeque::new();
    queue.push_back(start);

    macro_rules! check_cost {
        ($state: expr, $cost: expr) => {{
            let state = $state;
            let cost = $cost;
            if !costs.contains_key(&state) || costs[&state] > cost {
                costs.insert(state, cost);
                queue.push_back(state);
            }
        }};
    }

    while let Some(st) = queue.pop_front() {
        // consider all possible moves
        // move an amphipod into an empty space
        let cost = costs[&st];
        if st[7] != Amphipod::None {
            if st[1] == Amphipod::None {
                if st[0] == Amphipod::None {
                    check_cost!(st.swap(0, 7), cost + 3 * st[7].cost());
                }
                check_cost!(st.swap(1, 7), cost + 2 * st[7].cost());
            }
            if st[2] == Amphipod::None {
                check_cost!(st.swap(2, 7), cost + 2 * st[7].cost());
                if st[3] == Amphipod::None {
                    check_cost!(st.swap(3, 7), cost + 4 * st[7].cost());
                    if st[4] == Amphipod::None {
                        check_cost!(st.swap(4, 7), cost + 6 * st[7].cost());
                        if st[5] == Amphipod::None {
                            check_cost!(st.swap(5, 7), cost + 8 * st[7].cost());
                            if st[6] == Amphipod::None {
                                check_cost!(st.swap(6, 7), cost + 9 * st[7].cost());
                            }
                        }
                    }
                }
            }
            if st[11] == Amphipod::None {
                check_cost!(st.swap(11, 7), cost + st[7].cost());
            }
        } else if st[11] != Amphipod::None {
            if st[1] == Amphipod::None {
                if st[0] == Amphipod::None {
                    check_cost!(st.swap(0, 11), cost + 4 * st[11].cost());
                }
                check_cost!(st.swap(1, 11), cost + 3 * st[11].cost());
            }
            if st[2] == Amphipod::None {
                check_cost!(st.swap(2, 11), cost + 3 * st[11].cost());
                if st[3] == Amphipod::None {
                    check_cost!(st.swap(3, 11), cost + 5 * st[11].cost());
                    if st[4] == Amphipod::None {
                        check_cost!(st.swap(4, 11), cost + 7 * st[11].cost());
                        if st[5] == Amphipod::None {
                            check_cost!(st.swap(5, 11), cost + 9 * st[11].cost());
                            if st[6] == Amphipod::None {
                                check_cost!(st.swap(6, 11), cost + 10 * st[11].cost());
                            }
                        }
                    }
                }
            }
        }
        if st[8] != Amphipod::None {
            if st[2] == Amphipod::None {
                if st[1] == Amphipod::None {
                    if st[0] == Amphipod::None {
                        check_cost!(st.swap(0, 8), cost + 5 * st[8].cost());
                    }
                    check_cost!(st.swap(1, 8), cost + 4 * st[8].cost());
                }
                check_cost!(st.swap(2, 8), cost + 2 * st[8].cost());
            }
            if st[3] == Amphipod::None {
                check_cost!(st.swap(3, 8), cost + 2 * st[8].cost());
                if st[4] == Amphipod::None {
                    check_cost!(st.swap(4, 8), cost + 4 * st[8].cost());
                    if st[5] == Amphipod::None {
                        check_cost!(st.swap(5, 8), cost + 6 * st[8].cost());
                        if st[6] == Amphipod::None {
                            check_cost!(st.swap(6, 8), cost + 7 * st[8].cost());
                        }
                    }
                }
            }
            if st[12] == Amphipod::None {
                check_cost!(st.swap(12, 8), cost + st[8].cost());
            }
        } else if st[12] != Amphipod::None {
            if st[2] == Amphipod::None {
                if st[1] == Amphipod::None {
                    if st[0] == Amphipod::None {
                        check_cost!(st.swap(0, 12), cost + 6 * st[12].cost());
                    }
                    check_cost!(st.swap(1, 12), cost + 5 * st[12].cost());
                }
                check_cost!(st.swap(2, 12), cost + 3 * st[12].cost());
            }
            if st[3] == Amphipod::None {
                check_cost!(st.swap(3, 12), cost + 3 * st[12].cost());
                if st[4] == Amphipod::None {
                    check_cost!(st.swap(4, 12), cost + 5 * st[12].cost());
                    if st[5] == Amphipod::None {
                        check_cost!(st.swap(5, 12), cost + 7 * st[12].cost());
                        if st[6] == Amphipod::None {
                            check_cost!(st.swap(6, 12), cost + 8 * st[12].cost());
                        }
                    }
                }
            }
        }
        if st[9] != Amphipod::None {
            if st[3] == Amphipod::None {
                if st[2] == Amphipod::None {
                    if st[1] == Amphipod::None {
                        if st[0] == Amphipod::None {
                            check_cost!(st.swap(0, 9), cost + 7 * st[9].cost());
                        }
                        check_cost!(st.swap(1, 9), cost + 6 * st[9].cost());
                    }
                    check_cost!(st.swap(2, 9), cost + 4 * st[9].cost());
                }
                check_cost!(st.swap(3, 9), cost + 2 * st[9].cost());
            }
            if st[4] == Amphipod::None {
                check_cost!(st.swap(4, 9), cost + 2 * st[9].cost());
                if st[5] == Amphipod::None {
                    check_cost!(st.swap(5, 9), cost + 4 * st[9].cost());
                    if st[6] == Amphipod::None {
                        check_cost!(st.swap(6, 9), cost + 5 * st[9].cost());
                    }
                }
            }
            if st[13] == Amphipod::None {
                check_cost!(st.swap(13, 9), cost + st[9].cost());
            }
        } else if st[13] != Amphipod::None {
            if st[3] == Amphipod::None {
                if st[2] == Amphipod::None {
                    if st[1] == Amphipod::None {
                        if st[0] == Amphipod::None {
                            check_cost!(st.swap(0, 13), cost + 8 * st[13].cost());
                        }
                        check_cost!(st.swap(1, 13), cost + 7 * st[13].cost());
                    }
                    check_cost!(st.swap(2, 13), cost + 5 * st[13].cost());
                }
                check_cost!(st.swap(3, 13), cost + 3 * st[13].cost());
            }
            if st[4] == Amphipod::None {
                check_cost!(st.swap(4, 13), cost + 3 * st[13].cost());
                if st[5] == Amphipod::None {
                    check_cost!(st.swap(5, 13), cost + 5 * st[13].cost());
                    if st[6] == Amphipod::None {
                        check_cost!(st.swap(6, 13), cost + 6 * st[13].cost());
                    }
                }
            }
        }
        if st[10] != Amphipod::None {
            if st[4] == Amphipod::None {
                if st[3] == Amphipod::None {
                    if st[2] == Amphipod::None {
                        if st[1] == Amphipod::None {
                            if st[0] == Amphipod::None {
                                check_cost!(st.swap(0, 10), cost + 9 * st[10].cost());
                            }
                            check_cost!(st.swap(1, 10), cost + 8 * st[10].cost());
                        }
                        check_cost!(st.swap(2, 10), cost + 6 * st[10].cost());
                    }
                    check_cost!(st.swap(3, 10), cost + 4 * st[10].cost());
                }
                check_cost!(st.swap(4, 10), cost + 2 * st[10].cost());
            }
            if st[5] == Amphipod::None {
                check_cost!(st.swap(5, 10), cost + 2 * st[10].cost());
                if st[6] == Amphipod::None {
                    check_cost!(st.swap(6, 10), cost + 3 * st[10].cost());
                }
            }
            if st[14] == Amphipod::None {
                check_cost!(st.swap(14, 10), cost + st[10].cost());
            }
        } else if st[14] != Amphipod::None {
            if st[4] == Amphipod::None {
                if st[3] == Amphipod::None {
                    if st[2] == Amphipod::None {
                        if st[1] == Amphipod::None {
                            if st[0] == Amphipod::None {
                                check_cost!(st.swap(0, 14), cost + 10 * st[14].cost());
                            }
                            check_cost!(st.swap(1, 14), cost + 9 * st[14].cost());
                        }
                        check_cost!(st.swap(2, 14), cost + 7 * st[14].cost());
                    }
                    check_cost!(st.swap(3, 14), cost + 5 * st[14].cost());
                }
                check_cost!(st.swap(4, 14), cost + 3 * st[14].cost());
            }
            if st[5] == Amphipod::None {
                check_cost!(st.swap(5, 14), cost + 3 * st[14].cost());
                if st[6] == Amphipod::None {
                    check_cost!(st.swap(6, 14), cost + 4 * st[14].cost());
                }
            }
        }

        // move an amphipod into the destination room
        use Amphipod::{None as AN, A as AA, B as AB, C as AC, D as AD};
        if st[0] == AA {
            if st[1] == AN && st[7] == AN {
                check_cost!(st.swap(0, 7), cost + 3);
            }
        }
        if st[0] == AB {
            if st[1] == AN && st[2] == AN && st[8] == AN {
                check_cost!(st.swap(0, 8), cost + 50);
            }
        }
        if st[0] == AC {
            if st[1] == AN && st[2] == AN && st[3] == AN && st[9] == AN {
                check_cost!(st.swap(0, 9), cost + 700);
            }
        }
        if st[0] == AD {
            if st[1] == AN && st[2] == AN && st[3] == AN && st[4] == AN && st[10] == AN {
                check_cost!(st.swap(0, 10), cost + 9000);
            }
        }
        if st[1] == AA {
            if st[7] == AN {
                check_cost!(st.swap(1, 7), cost + 2);
            }
        }
        if st[1] == AB {
            if st[2] == AN && st[8] == AN {
                check_cost!(st.swap(1, 8), cost + 40);
            }
        }
        if st[1] == AC {
            if st[2] == AN && st[3] == AN && st[9] == AN {
                check_cost!(st.swap(1, 9), cost + 600);
            }
        }
        if st[1] == AD {
            if st[2] == AN && st[3] == AN && st[4] == AN && st[10] == AN {
                check_cost!(st.swap(1, 10), cost + 8000);
            }
        }
        if st[2] == AA {
            if st[7] == AN {
                check_cost!(st.swap(2, 7), cost + 2);
            }
        }
        if st[2] == AB {
            if st[8] == AN {
                check_cost!(st.swap(2, 8), cost + 20);
            }
        }
        if st[2] == AC {
            if st[3] == AN && st[9] == AN {
                check_cost!(st.swap(2, 9), cost + 400);
            }
        }
        if st[2] == AD {
            if st[3] == AN && st[4] == AN && st[10] == AN {
                check_cost!(st.swap(2, 10), cost + 6000);
            }
        }
        if st[3] == AA {
            if st[2] == AN && st[7] == AN {
                check_cost!(st.swap(3, 7), cost + 4);
            }
        }
        if st[3] == AB {
            if st[8] == AN {
                check_cost!(st.swap(3, 8), cost + 20);
            }
        }
        if st[3] == AC {
            if st[9] == AN {
                check_cost!(st.swap(3, 9), cost + 200);
            }
        }
        if st[3] == AD {
            if st[4] == AN && st[10] == AN {
                check_cost!(st.swap(3, 10), cost + 4000);
            }
        }
        if st[4] == AA {
            if st[3] == AN && st[2] == AN && st[7] == AN {
                check_cost!(st.swap(4, 7), cost + 6);
            }
        }
        if st[4] == AB {
            if st[3] == AN && st[8] == AN {
                check_cost!(st.swap(4, 8), cost + 40);
            }
        }
        if st[4] == AC {
            if st[9] == AN {
                check_cost!(st.swap(4, 9), cost + 200);
            }
        }
        if st[4] == AD {
            if st[10] == AN {
                check_cost!(st.swap(4, 10), cost + 2000);
            }
        }
        if st[5] == AA {
            if st[4] == AN && st[3] == AN && st[2] == AN && st[7] == AN {
                check_cost!(st.swap(5, 7), cost + 8);
            }
        }
        if st[5] == AB {
            if st[4] == AN && st[3] == AN && st[8] == AN {
                check_cost!(st.swap(5, 8), cost + 60);
            }
        }
        if st[5] == AC {
            if st[4] == AN && st[9] == AN {
                check_cost!(st.swap(5, 9), cost + 400);
            }
        }
        if st[5] == AD {
            if st[10] == AN {
                check_cost!(st.swap(5, 10), cost + 2000);
            }
        }
        if st[6] == AA {
            if st[5] == AN && st[4] == AN && st[3] == AN && st[2] == AN && st[7] == AN {
                check_cost!(st.swap(6, 7), cost + 9);
            }
        }
        if st[6] == AB {
            if st[5] == AN && st[4] == AN && st[3] == AN && st[8] == AN {
                check_cost!(st.swap(6, 8), cost + 70);
            }
        }
        if st[6] == AC {
            if st[5] == AN && st[4] == AN && st[9] == AN {
                check_cost!(st.swap(6, 9), cost + 500);
            }
        }
        if st[6] == AD {
            if st[5] == AN && st[10] == AN {
                check_cost!(st.swap(6, 10), cost + 3000);
            }
        }
    }

    // find the price of the final state
    let mut target = [Amphipod::None; 15];
    for i in [7, 11] {
        target[i] = Amphipod::A;
        target[i + 1] = Amphipod::B;
        target[i + 2] = Amphipod::C;
        target[i + 3] = Amphipod::D;
    }
    let target = State(target);

    // display the result
    print!("{}, ", costs[&target]);
    io::stdout().flush().unwrap();

    // an extended amphipod configuration
    // 0  1  x  2  x  3  x  4  x  5  6
    //       7     8     9    10
    //      11    12    13    14
    //      15    16    17    18
    //      19    20    21    22

    #[derive(Eq, Copy, Clone, Debug)]
    struct StateExt {
        state: [Amphipod; 23],
        cost: usize,
    }
    impl StateExt {
        fn swap(&self, a: usize, b: usize, c: usize) -> StateExt {
            let mut ret = self.clone();
            ret[a] = self[b];
            ret[b] = self[a];
            ret.cost = c;
            ret
        }
    }
    impl std::ops::Index<usize> for StateExt {
        type Output = Amphipod;
        fn index(&self, i: usize) -> &Amphipod {
            &self.state[i]
        }
    }
    impl std::ops::IndexMut<usize> for StateExt {
        fn index_mut(&mut self, i: usize) -> &mut Amphipod {
            &mut self.state[i]
        }
    }
    impl PartialEq for StateExt {
        fn eq(&self, other: &Self) -> bool {
            self.cost == other.cost
        }
    }
    impl PartialOrd for StateExt {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            other.cost.partial_cmp(&self.cost)
        }
    }
    impl Ord for StateExt {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost)
        }
    }

    // starting configuration
    let mut start = [Amphipod::None; 23];
    for i in 0..4 {
        start[7 + i] = Amphipod::from_char(rooms[0][i]);
        start[11 + i] = Amphipod::from_char(rooms[1][i]);
        start[15 + i] = Amphipod::from_char(rooms[2][i]);
        start[19 + i] = Amphipod::from_char(rooms[3][i]);
    }

    // costs
    let mut costs = HashMap::new();

    // binary heap of states to consider
    let mut heap = BinaryHeap::new();
    let start = StateExt {
        state: start,
        cost: 0,
    };
    heap.push(start);
    let mut prev_cost = 0;

    macro_rules! check_cost {
        ($state: expr, $a: expr, $b: expr, $c: expr) => {{
            let cost = $c;
            let state = $state.swap($a, $b, cost);
            if !costs.contains_key(&state.state) || costs[&state.state] > cost {
                heap.push(state);
            }
        }};
    }

    while let Some(st) = heap.pop() {
        // consider all possible moves
        let cost = costs.get(&st.state);
        if let Some(cost) = cost {
            if *cost <= st.cost {
                continue
            } else {
                costs.insert(st.state.clone(), st.cost);
            }
        } else {
            costs.insert(st.state.clone(), st.cost);
        }
        let cost = st.cost;
        if cost > prev_cost + 1000 {
            prev_cost = cost;
            println!("{}", cost);
        }
        if cost > 50000 {
            break;
        }

        // move an amphipod into an empty space
        if st[7] != Amphipod::None {
            if st[1] == Amphipod::None {
                if st[0] == Amphipod::None {
                    check_cost!(st, 0, 7, cost + 3 * st[7].cost());
                }
                check_cost!(st, 1, 7, cost + 2 * st[7].cost());
            }
            if st[2] == Amphipod::None {
                check_cost!(st, 2, 7, cost + 2 * st[7].cost());
                if st[3] == Amphipod::None {
                    check_cost!(st, 3, 7, cost + 4 * st[7].cost());
                    if st[4] == Amphipod::None {
                        check_cost!(st, 4, 7, cost + 6 * st[7].cost());
                        if st[5] == Amphipod::None {
                            check_cost!(st, 5, 7, cost + 8 * st[7].cost());
                            if st[6] == Amphipod::None {
                                check_cost!(st, 6, 7, cost + 9 * st[7].cost());
                            }
                        }
                    }
                }
            }
            if st[11] == Amphipod::None {
                check_cost!(st, 11, 7, cost + st[7].cost());
                if st[15] == Amphipod::None {
                    check_cost!(st, 15, 7, cost + 2 * st[7].cost());
                    if st[19] == Amphipod::None {
                        check_cost!(st, 19, 7, cost + 3 * st[7].cost());
                    }
                }
            }
        } else if st[11] != Amphipod::None {
            check_cost!(st, 7, 11, cost + st[11].cost());
        } else if st[15] != Amphipod::None {
            check_cost!(st, 7, 15, cost + 2 * st[15].cost());
        } else if st[19] != Amphipod::None {
            check_cost!(st, 7, 19, cost + 3 * st[19].cost());
        }
        if st[8] != Amphipod::None {
            if st[2] == Amphipod::None {
                if st[1] == Amphipod::None {
                    if st[0] == Amphipod::None {
                        check_cost!(st, 0, 8, cost + 5 * st[8].cost());
                    }
                    check_cost!(st, 1, 8, cost + 4 * st[8].cost());
                }
                check_cost!(st, 2, 8, cost + 2 * st[8].cost());
            }
            if st[3] == Amphipod::None {
                check_cost!(st, 3, 8, cost + 2 * st[8].cost());
                if st[4] == Amphipod::None {
                    check_cost!(st, 4, 8, cost + 4 * st[8].cost());
                    if st[5] == Amphipod::None {
                        check_cost!(st, 5, 8, cost + 6 * st[8].cost());
                        if st[6] == Amphipod::None {
                            check_cost!(st, 6, 8, cost + 7 * st[8].cost());
                        }
                    }
                }
            }
            if st[12] == Amphipod::None {
                check_cost!(st, 12, 8, cost + st[8].cost());
                if st[16] == Amphipod::None {
                    check_cost!(st, 16, 8, cost + 2 * st[8].cost());
                    if st[20] == Amphipod::None {
                        check_cost!(st, 20, 8, cost + 3 * st[8].cost());
                    }
                }
            }
        } else if st[12] != Amphipod::None {
            check_cost!(st, 8, 12, cost + st[12].cost());
        } else if st[16] != Amphipod::None {
            check_cost!(st, 8, 16, cost + 2 * st[16].cost());
        } else if st[20] != Amphipod::None {
            check_cost!(st, 8, 20, cost + 3 * st[20].cost());
        }
        if st[9] != Amphipod::None {
            if st[3] == Amphipod::None {
                if st[2] == Amphipod::None {
                    if st[1] == Amphipod::None {
                        if st[0] == Amphipod::None {
                            check_cost!(st, 0, 9, cost + 7 * st[9].cost());
                        }
                        check_cost!(st, 1, 9, cost + 6 * st[9].cost());
                    }
                    check_cost!(st, 2, 9, cost + 4 * st[9].cost());
                }
                check_cost!(st, 3, 9, cost + 2 * st[9].cost());
            }
            if st[4] == Amphipod::None {
                check_cost!(st, 4, 9, cost + 2 * st[9].cost());
                if st[5] == Amphipod::None {
                    check_cost!(st, 5, 9, cost + 4 * st[9].cost());
                    if st[6] == Amphipod::None {
                        check_cost!(st, 6, 9, cost + 5 * st[9].cost());
                    }
                }
            }
            if st[13] == Amphipod::None {
                check_cost!(st, 13, 9, cost + st[9].cost());
                if st[17] == Amphipod::None {
                    check_cost!(st, 17, 9, cost + 2 * st[9].cost());
                    if st[21] == Amphipod::None {
                        check_cost!(st, 21, 9, cost + 3 * st[9].cost());
                    }
                }
            }
        } else if st[13] != Amphipod::None {
            check_cost!(st, 9, 13, cost + st[13].cost());
        } else if st[17] != Amphipod::None {
            check_cost!(st, 9, 17, cost + 2 * st[17].cost());
        } else if st[21] != Amphipod::None {
            check_cost!(st, 9, 21, cost + 3 * st[21].cost());
        }
        if st[10] != Amphipod::None {
            if st[4] == Amphipod::None {
                if st[3] == Amphipod::None {
                    if st[2] == Amphipod::None {
                        if st[1] == Amphipod::None {
                            if st[0] == Amphipod::None {
                                check_cost!(st, 0, 10, cost + 9 * st[10].cost());
                            }
                            check_cost!(st, 1, 10, cost + 8 * st[10].cost());
                        }
                        check_cost!(st, 2, 10, cost + 6 * st[10].cost());
                    }
                    check_cost!(st, 3, 10, cost + 4 * st[10].cost());
                }
                check_cost!(st, 4, 10, cost + 2 * st[10].cost());
            }
            if st[5] == Amphipod::None {
                check_cost!(st, 5, 10, cost + 2 * st[10].cost());
                if st[6] == Amphipod::None {
                    check_cost!(st, 6, 10, cost + 3 * st[10].cost());
                }
            }
            if st[14] == Amphipod::None {
                check_cost!(st, 14, 10, cost + st[10].cost());
                if st[18] == Amphipod::None {
                    check_cost!(st, 18, 10, cost + 2 * st[10].cost());
                    if st[22] == Amphipod::None {
                        check_cost!(st, 22, 10, cost + 3 * st[10].cost());
                    }
                }
            }
        } else if st[14] != Amphipod::None {
            check_cost!(st, 10, 14, cost + st[14].cost());
        } else if st[18] != Amphipod::None {
            check_cost!(st, 10, 18, cost + 2 * st[18].cost());
        } else if st[22] != Amphipod::None {
            check_cost!(st, 10, 22, cost + 3 * st[22].cost());
        }

        // move an amphipod into the destination room
        use Amphipod::{None as AN, A as AA, B as AB, C as AC, D as AD};
        if st[0] == AA {
            if st[1] == AN && st[7] == AN {
                check_cost!(st, 0, 7, cost + 3);
            }
        }
        if st[0] == AB {
            if st[1] == AN && st[2] == AN && st[8] == AN {
                check_cost!(st, 0, 8, cost + 50);
            }
        }
        if st[0] == AC {
            if st[1] == AN && st[2] == AN && st[3] == AN && st[9] == AN {
                check_cost!(st, 0, 9, cost + 700);
            }
        }
        if st[0] == AD {
            if st[1] == AN && st[2] == AN && st[3] == AN && st[4] == AN && st[10] == AN {
                check_cost!(st, 0, 10, cost + 9000);
            }
        }
        if st[1] == AA {
            if st[7] == AN {
                check_cost!(st, 1, 7, cost + 2);
            }
        }
        if st[1] == AB {
            if st[2] == AN && st[8] == AN {
                check_cost!(st, 1, 8, cost + 40);
            }
        }
        if st[1] == AC {
            if st[2] == AN && st[3] == AN && st[9] == AN {
                check_cost!(st, 1, 9, cost + 600);
            }
        }
        if st[1] == AD {
            if st[2] == AN && st[3] == AN && st[4] == AN && st[10] == AN {
                check_cost!(st, 1, 10, cost + 8000);
            }
        }
        if st[2] == AA {
            if st[7] == AN {
                check_cost!(st, 2, 7, cost + 2);
            }
        }
        if st[2] == AB {
            if st[8] == AN {
                check_cost!(st, 2, 8, cost + 20);
            }
        }
        if st[2] == AC {
            if st[3] == AN && st[9] == AN {
                check_cost!(st, 2, 9, cost + 400);
            }
        }
        if st[2] == AD {
            if st[3] == AN && st[4] == AN && st[10] == AN {
                check_cost!(st, 2, 10, cost + 6000);
            }
        }
        if st[3] == AA {
            if st[2] == AN && st[7] == AN {
                check_cost!(st, 3, 7, cost + 4);
            }
        }
        if st[3] == AB {
            if st[8] == AN {
                check_cost!(st, 3, 8, cost + 20);
            }
        }
        if st[3] == AC {
            if st[9] == AN {
                check_cost!(st, 3, 9, cost + 200);
            }
        }
        if st[3] == AD {
            if st[4] == AN && st[10] == AN {
                check_cost!(st, 3, 10, cost + 4000);
            }
        }
        if st[4] == AA {
            if st[3] == AN && st[2] == AN && st[7] == AN {
                check_cost!(st, 4, 7, cost + 6);
            }
        }
        if st[4] == AB {
            if st[3] == AN && st[8] == AN {
                check_cost!(st, 4, 8, cost + 40);
            }
        }
        if st[4] == AC {
            if st[9] == AN {
                check_cost!(st, 4, 9, cost + 200);
            }
        }
        if st[4] == AD {
            if st[10] == AN {
                check_cost!(st, 4, 10, cost + 2000);
            }
        }
        if st[5] == AA {
            if st[4] == AN && st[3] == AN && st[2] == AN && st[7] == AN {
                check_cost!(st, 5, 7, cost + 8);
            }
        }
        if st[5] == AB {
            if st[4] == AN && st[3] == AN && st[8] == AN {
                check_cost!(st, 5, 8, cost + 60);
            }
        }
        if st[5] == AC {
            if st[4] == AN && st[9] == AN {
                check_cost!(st, 5, 9, cost + 400);
            }
        }
        if st[5] == AD {
            if st[10] == AN {
                check_cost!(st, 5, 10, cost + 2000);
            }
        }
        if st[6] == AA {
            if st[5] == AN && st[4] == AN && st[3] == AN && st[2] == AN && st[7] == AN {
                check_cost!(st, 6, 7, cost + 9);
            }
        }
        if st[6] == AB {
            if st[5] == AN && st[4] == AN && st[3] == AN && st[8] == AN {
                check_cost!(st, 6, 8, cost + 70);
            }
        }
        if st[6] == AC {
            if st[5] == AN && st[4] == AN && st[9] == AN {
                check_cost!(st, 6, 9, cost + 500);
            }
        }
        if st[6] == AD {
            if st[5] == AN && st[10] == AN {
                check_cost!(st, 6, 10, cost + 3000);
            }
        }
    }

    // find the price of the final state
    let mut target = [Amphipod::None; 23];
    for i in [7, 11, 15, 19] {
        target[i] = Amphipod::A;
        target[i + 1] = Amphipod::B;
        target[i + 2] = Amphipod::C;
        target[i + 3] = Amphipod::D;
    }

    // display the result
    println!("{}", costs[&target]);
}

fn day_24() {
    // The following was pretty much unused:
    // read and parse the data
    let data = fs::read_to_string("inputs/day_24.in").expect("aaa");
    let _monad = data
        .lines()
        .map(|l| l.trim().split(" ").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // ALU simulator
    fn simulate<T: Iterator<Item = i64>>(instructions: &Vec<Vec<&str>>, mut input: T) -> [i64; 4] {
        let mut ret = [0; 4];
        let reg_idx = |x: &str| -> (bool, i64) {
            match x {
                "w" => (true, 0),
                "x" => (true, 1),
                "y" => (true, 2),
                "z" => (true, 3),
                _ => (false, x.parse::<i64>().unwrap()),
            }
        };
        for inst in instructions {
            match inst[0] {
                "inp" => {
                    if let (true, reg) = reg_idx(inst[1]) {
                        ret[reg as usize] = input.next().unwrap();
                    }
                }
                "add" => {
                    if let (true, reg) = reg_idx(inst[1]) {
                        let (direct, val) = reg_idx(inst[2]);
                        if direct {
                            ret[reg as usize] += ret[val as usize];
                        } else {
                            ret[reg as usize] += val;
                        }
                    }
                }
                "mul" => {
                    if let (true, reg) = reg_idx(inst[1]) {
                        let (direct, val) = reg_idx(inst[2]);
                        if direct {
                            ret[reg as usize] *= ret[val as usize];
                        } else {
                            ret[reg as usize] *= val;
                        }
                    }
                }
                "div" => {
                    if let (true, reg) = reg_idx(inst[1]) {
                        let (direct, val) = reg_idx(inst[2]);
                        if direct {
                            ret[reg as usize] /= ret[val as usize];
                        } else {
                            ret[reg as usize] /= val;
                        }
                    }
                }
                "mod" => {
                    if let (true, reg) = reg_idx(inst[1]) {
                        let (direct, val) = reg_idx(inst[2]);
                        if direct {
                            ret[reg as usize] %= ret[val as usize];
                        } else {
                            ret[reg as usize] %= val;
                        }
                    }
                }
                "eql" => {
                    if let (true, reg) = reg_idx(inst[1]) {
                        let (direct, val) = reg_idx(inst[2]);
                        if direct {
                            ret[reg as usize] = (ret[reg as usize] == ret[val as usize]) as i64;
                        } else {
                            ret[reg as usize] = (ret[reg as usize] == val) as i64;
                        }
                    }
                }
                _ => (),
            }
        }
        ret
    }

    // let negate = vec![vec!["inp", "x"], vec!["mul", "x", "-1"]];
    // println!("{:?}", simulate(&negate, once(13)));

    // let binary = vec![
    //     vec!["inp", "w"],
    //     vec!["add", "z", "w"],
    //     vec!["mod", "z", "2"],
    //     vec!["div", "w", "2"],
    //     vec!["add", "y", "w"],
    //     vec!["mod", "y", "2"],
    //     vec!["div", "w", "2"],
    //     vec!["add", "x", "w"],
    //     vec!["mod", "x", "2"],
    //     vec!["div", "w", "2"],
    //     vec!["mod", "w", "2"]
    // ];
    // println!("{:?}", simulate(&binary, once(13)));

    let _digits = || (0..9).map(|x| 9 - x);
    // let mut largest_model_num = -1_i64;
    // let mut input = [9_i64; 14];
    // 'outer: for d7 in digits() {
    //     input[6] = d7;
    //     for d6 in digits() {
    //         input[7] = d6;
    //         for d5 in digits() {
    //             input[8] = d5;
    //             for d4 in digits() {
    //                 input[9] = d4;
    //                 for d3 in digits() {
    //                     input[10] = d3;
    //                     for d2 in digits() {
    //                         input[11] = d2;
    //                         for d1 in digits() {
    //                             input[12] = d1;
    //                             for d0 in digits() {
    //                                 input[13] = d0;
    //                                 let ret = simulate(&monad, input.clone().into_iter());
    //                                 if ret[3] == 0 {
    //                                     largest_model_num = 0;
    //                                     for i in 0..14 {
    //                                         largest_model_num *= 10;
    //                                         largest_model_num += ret[i];
    //                                         break 'outer;
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    // if largest_model_num == -1 {
    //     panic!("Not found");
    // }

    // let input = [2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2];
    // let _ = simulate(&monad, input.into_iter());

    // At this point I realized I'd have to find a more optimal approach.
    // I studied the MONAD and found that it's composed of 14 almost identical blocks
    //   (one for each input).
    // I also noticed that the z register acs like a stack of numbers from 0 to 25.
    // Therefore the MONAD acts like this:

    // x = peek()
    // if (x + 11) != char { push(14 + char) }
    // x = peek()
    // if (x + 13) != char { push(8 + char) }
    // x = peek()
    // if (x + 11) != char { push(4 + char) }
    // x = peek()
    // if (x + 10) != char { push(10 + char) }
    // x = pop()
    // if (x - 3) != char { push(14 + char) }
    // x = pop()
    // if (x - 4) != char { push(10 + char) }
    // x = peek()
    // if (x + 12) != char { push(4 + char) }
    // x = pop()
    // if (x - 8) != char { push(14 + char) }
    // x = pop()
    // if (x - 3) != char { push(1 + char) }
    // x = pop()
    // if (x - 12) != char { push(6 + char) }
    // x = peek()
    // if (x + 14) != char { push(char) }
    // x = pop()
    // if (x - 6) != char { push(9 + char) }
    // x = peek()
    // if (x + 11) != char { push(13 + char) }
    // x = pop()
    // if (x - 12) != char { push(12 + char) }

    // In order to make the value of z be 0, we must make sure the stack is empty.
    // I noticed that there are 7 pops and 14 pushes.
    // But some pushes can be avoided, those whose conditional can be achieved.
    // Since x is always between 0 and 25, and char is between 1 and 9, this immediatelly
    //   rules out the conditionals with positive offsets over 9.
    // With this the MONAD becomes:

    // push(c0+14)
    // push(c1+8)
    // push(c2+4)
    // push(c3+4)
    // x = c3+10
    // if (c3+7) != c4 { push(c4+14) }
    // x = c2+4
    // if (c2) != c5 { push(c5+10) }
    // push(c6+4)
    // x = c6+4
    // if (c6-4) != c7 { push(c7+14) }
    // x = c1+8
    // if (c1+5) != c8 { push(c8+1) }
    // x = c0+14
    // if (c0+2) != c9 { push(c9+6) }
    // push(c10)
    // x = c10
    // if (c10-6) != c11 { push(c11+9) }
    // push(c12+13)
    // x = c12+13
    // if (c12+1) != c13 { push(c13+12) }

    // To achieve empty stack, we must satisfy 7 equations:

    // c4 = c3+7
    // c5 = c2
    // c7 = c6-4
    // c8 = c1+5
    // c9 = c0+2
    // c11 = c10-6
    // c13 = c12+1

    // Now we just insert values from 1 to 9 to get largest and smallest valid model number:
    println!("{}, {}", 74929995999389_i64, 11118151637112_i64);
}

fn day_25() {
    // read and parse the data
    let data = fs::read_to_string("inputs/day_25.in").expect("aaa");
    #[derive(Copy, Clone, PartialEq)]
    enum Cucumber {
        Empty,
        East,
        South,
    }
    use Cucumber::*;
    let starting_field = data
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '>' => East,
                    'v' => South,
                    _ => Empty,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // apply movement until no cucumber moves
    let mut moves = 1;
    let mut prev_field = starting_field.clone();
    loop {
        let mut moved = 0;
        let mut new_field = vec![vec![Empty; prev_field[0].len()]; prev_field.len()];

        // move east
        let l_i = prev_field.len();
        for i in 0..l_i {
            let l_j = prev_field[i].len();
            for j in 0..l_j {
                if prev_field[i][j] == East {
                    if prev_field[i][(j + 1) % l_j] == Empty {
                        new_field[i][(j + 1) % l_j] = East;
                        moved += 1;
                    } else {
                        new_field[i][j] = East;
                    }
                }
                if prev_field[i][j] == South {
                    new_field[i][j] = South;
                }
            }
        }

        prev_field = new_field;
        let mut new_field = vec![vec![Empty; prev_field[0].len()]; prev_field.len()];

        // move south
        let l_i = prev_field.len();
        for i in 0..l_i {
            let l_j = prev_field[i].len();
            for j in 0..l_j {
                if prev_field[i][j] == South {
                    if prev_field[(i + 1) % l_i][j] == Empty {
                        new_field[(i + 1) % l_i][j] = South;
                        moved += 1;
                    } else {
                        new_field[i][j] = South;
                    }
                }
                if prev_field[i][j] == East {
                    new_field[i][j] = East;
                }
            }
        }

        // check if any cucumber moved
        if moved == 0 {
            break;
        }

        // cleanup
        prev_field = new_field;
        moves += 1;
    }

    // display the result
    println!("{}, {}", moves, 0);
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
    // day_10();
    // day_11();
    // day_12();
    // day_13();
    // day_14();
    // day_15();
    // day_16();
    // day_17();
    // day_18();
    // day_19();
    // day_20();
    // day_21();
    // day_22();
    day_23();
    // day_24();
    // day_25()
}
