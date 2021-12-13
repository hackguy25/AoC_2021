# Solutions for Advent of Code 2021

This is a collection of compiled solutions for [Advent of Code 2021](https://adventofcode.com/) as the puzzles were solved originally.
The solutions are written in [Rust](https://www.rust-lang.org/).

I have decided to use this opportunity to learn about the Rust programming language.
Since I hope to learn something new with each puzzle, I'm keeping all solutions mostly untouched after the result submission.
This will allow me to consider the progress I made between each puzzle.
Since I might get new insights or ideas for an old solution later down the road, I'll keep them in a separate file, `snippets.rs`.

## What I learned

### Day 1
- Reading a text file into a `String`: `std::fs::read_to_string`

### Day 2
- Iterator over the lines in a `String`: `s.lines()`
- Map a function/closure over an iterator, discard failures: `it.filter_map(f)`
- Type hint only for outer type parameter:
  - `let y: Vec<_> = x.split(" ").collect();`
  - `let y = x.split(" ").collect::Vec<_>();`
- Chain fallible operations with `and_then`:
  `y[1].parse::<i32>().ok().and_then(|n| y[0].chars().nth(0).and_then(|c| Some((c, n))))`

### Day 3
- `if` statement as ternary operator:
  `counts[i] += if chars.next() == Some('1') { 1 } else { -1 }`
- Parse numbers with different bases: `i32::from_str_radix("1011101", 2).unwrap()`

### Day 4
- Ranges can be used as iterators: `(0..5).map(|x| 2 * x - 1).collect::<Vec<_>>()`
- Blocks can be given labels and broken out of (see `day_04`)
- Variables can be declared without type and value if they're not going to be read before first assignment: `let a; a = 3 * something;`

### Day 5
- Large `Vec` initialization using a macro: `vec![vec![0, 1000]; 1000]`
- Argument destructuring in closures: `(&lines).into_iter().filter(|(a,b,c,d)| a == c || b == d)`
- Negative numbers can't easily be added to `usize`: `if dx * dy > 0 { y += 1; } else { y -= 1; }`

### Day 6
- Difference between `into_iter`, `iter` and `iter_mut`

### Day 7
- Remove whitespace around String: `trim`
- Custom max function on an iterator: `max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())`
- Iterate over elements in nested iterators: `flat_map`
- Shorten `map` and `filter map`: `it.map(f: x -> Result<y, _>).filter_map(Result::ok)` → `it.flat_map(f)`
  - `Result` is an iterator that yields one value if `Ok(x)` and no values if `Err(e)`

### Day 8
- Set operations using `HashSet`: `intersection`, equality
- `sort` vs. `sort_unstable`
- `iter.position(x)`
- `vec.swap(i, j)`
- `iter1.zip(iter2)`

### Day 9
- Parse `char` into its digit value: `c.to_digit(basis).unwrap()`
- Closures cannot be called recursively
- Only consider first `k` elements of an iterator: `iter.take(k)`

### Day 10
- Extend existing `vec` with and iterator: `v.extend(iter)`
- Labels let you break out of nested loops: `'outer: for i in ... { ... break 'outer; ... }`

### Day 12
- Anonymous values can be used in function calls that need a reference: `f(&mut HashMap::new())`
- String slices can be converted into owned strings directly: `"foo".to_owned()`

### Day 13
- Closure parameters can't be easily given lifetimes