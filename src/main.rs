#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet};
use std::fs;
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
    day_21();
}
