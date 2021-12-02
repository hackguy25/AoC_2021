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