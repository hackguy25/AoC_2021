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
