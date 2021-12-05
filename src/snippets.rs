fn day_02_alt() {
    // read and parse data
    let data = fs::read_to_string("inputs/day_02.in").expect("aaa");
    // let data = data
    //     .lines()
    //     .filter_map(|x| {
    //         let y: Vec<_> = x.split(" ").collect();
    //         y[1].parse::<i32>()
    //             .ok()
    //             .and_then(|n| y[0].chars().nth(0).and_then(|c| Some((c, n))))
    //     })
    //     .collect::<Vec<_>>();
    let data = data
        .lines()
        .map(|x| {
            let y: Vec<_> = x.split(" ").collect();
            (y[0].chars().nth(0).unwrap(), y[1].parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    // compute submarine's path
    let (mut depth, mut horiz) = (0, 0);
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
    let (mut aim, mut depth) = (0, 0);
    for (c, n) in &data {
        match c {
            'f' => depth += aim * n,
            'd' => aim += n,
            'u' => aim -= n,
            _ => (),
        }
    }

    // print result
    println!("({}, {})", first, depth * horiz);
}

fn day_03_alt() {
    // read data
    let data = fs::read_to_string("inputs/day_03.in").expect("aaa");

    // count appearances
    let mut counts = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for line in data.lines() {
        let mut chars = line.chars();
        for i in 0..12 {
            counts[i] += -1 + 2 * (chars.next() == Some('1')) as i32;
        }
    }

    // calculate epsilon and gamma
    let gamma = counts.into_iter().fold(0, |a, x| 2 * a + (x >= 0) as i32);
    let epsilon = counts.into_iter().fold(0, |a, x| 2 * a + (x < 0) as i32);

    // determine oxygen generator rating
    let mut values = data.lines().collect::<Vec<_>>();
    for i in 0..12 {
        let count = (&values)
            .into_iter()
            .fold(0, |a, x| a - 1 + 2 * (x.chars().nth(i) == Some('1')) as i32);
        values = values
            .into_iter()
            .filter(|s| s.chars().nth(i) == Some(if count >= 0 { '1' } else { '0' }))
            .collect();
    }
    let oxygen = i32::from_str_radix(values[0], 2).unwrap();

    // determine CO2 scrubber rating
    let mut values = data.lines().collect::<Vec<_>>();
    for i in 0..12 {
        let count = (&values)
            .into_iter()
            .fold(0, |a, x| a - 1 + 2 * (x.chars().nth(i) == Some('1')) as i32);
        values = values
            .into_iter()
            .filter(|s| s.chars().nth(i) == Some(if count < 0 { '1' } else { '0' }))
            .collect();
        if values.len() == 1 {
            break;
        }
    }
    let co2 = i32::from_str_radix(values[0], 2).unwrap();

    // print result
    println!("{}, {}", gamma * epsilon, oxygen * co2);
}

fn day_05_alt() {
    macro_rules! next {
        ($data: ident) => {
            $data.next().unwrap()
        };
    }

    // read and parse data
    let data = fs::read_to_string("inputs/day_05.in").expect("aaa");
    let lines = data
        .lines()
        .map(|line| {
            let mut data = line
                .split(" -> ")
                .map(|n| n.split(",").map(|x| x.parse::<usize>().unwrap()))
                .flatten();
            (next!(data), next!(data), next!(data), next!(data))
        })
        .collect::<Vec<_>>();

    // consider only vertical and horizontal lines
    let general = (&lines).into_iter().filter(|(a, b, c, d)| a == c || b == d);

    // plot general lines on a field
    let mut field = vec![vec![0; 1000]; 1000];
    for (a, b, c, d) in general {
        if a == c {
            for i in usize::min(*b, *d)..(usize::max(*b, *d) + 1) {
                field[*a][i] += 1;
            }
        } else {
            for i in usize::min(*a, *c)..(usize::max(*a, *c) + 1) {
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
            for i in usize::min(*b, *d)..(usize::max(*b, *d) + 1) {
                field[*a][i] += 1;
            }
        } else if b == d {
            for i in usize::min(*a, *c)..(usize::max(*a, *c) + 1) {
                field[i][*b] += 1;
            }
        } else {
            let mut y = if a < c { *b } else { *d };
            for i in usize::min(*a, *c)..(usize::max(*a, *c) + 1) {
                field[i][y] += 1;
                if 2 * (c > a) as i32 - 1 == 2 * (d > b) as i32 - 1 {
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
