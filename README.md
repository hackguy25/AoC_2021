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
- Shorten `map` and `filter map`: `it.map(f: x -> Result<y, _>).filter_map(Result::ok)` ??? `it.flat_map(f)`
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

### Day 14
- Hash maps/sets can take `&str`s as keys
- `String`s can be constructed from `char` arrays: `String::from_iter([key.chars().nth(0).unwrap(), *c])`
  - also works by collecting: `vec_of_chars.into_iter().collect::<String>()`

### Day 15
- Iterating over a changing container: `while let Some(x) = container.pop() { ... }`
- Multiple elements for a `Vec` cannot be borrowed if one borrow is mutable
- A queue: `let mut queue = VecDeque; queue.push_back(x); queue.pop_front()`

### Day 16
- Closure can take ownership of values using `move`: `let a = 3; let c = move || a + 3;`
- Strings can be formatted using format strings (see `std::fmt`)
- Functions that would consume an iterator can instead only take elements by passing `it.by_ref()`:
  - Take 5 elements into a `vec`: `it.by_ref().take(5).collect::<Vec<_>>()`
- Type trait's associated types can be further bounded: `fn evaluate<I: Iterator<Item = u8>>(...) ...`??

### Day 17
- Rust doesn't have named tuples, structs must be used
- Binary operators can be implemented for custom types by implementing traits in `std::ops`

### Day 18
- Garbage collection can be emulated using a reference-counting pointer: `std::rc::Rc`
- Structs/Enums can't have nested types defined inside
- Enum variants can be brought into scope: `enum Foo { A(i32), B, ... }; use Foo::*;`
- Debug strings can be derived or implemented by hand for new types
- Dereferenced values can be borrowed: `let (l, r) = &**p;`
- Functions can be implemented to extract enum variants directly: `impl Foo { fn a(self) -> i32 { if let Foo::A(n) = self { n } else { panic!("Foo not A") } } }`
- Ranges can be exclusive (`a..b`) or inclusive (`a..=b`) at the end

### Day 19
- If the size is known in advance: `let a: [i32; 3] = vec.as_slice().try_into().unwrap();`
- Destructuring assignments: `let x, y, z; let a = [1, 2, 3]; [x, y, z] = a;`
  - At the time of writing an unstable features, to be added in Rust 1.59.0

### Day 20
- `vec` macro can be used to initialize large vecs: `let a = vec![0; 1024];`
- Useful iterator functions:
  - `std::iter::once(x)` returns `x` once
  - `std::iter::repeat(x)` returns `x` forever
  - `it1.chain(it2)` first returns all values from `it1` and then all values from `it2`
  - `it1.zip(it2)` returns an iterator returning tuples `(x1, x2)` while both iterators return values
  - `it.cycle()` repeatedly returns all values from `it`, repeating from the beginning
  - `it.enumerate()` returns enumerated elements from `it`
  - `it.flatten()` returns elements of elements of `it`
  - `it.fold(init, f)` reduces `it` by `acc = init; loop { acc = f(acc, it.next()) }`
  - `it.intersperse(x)` alternates between elements of `it` and returning `x`
  - `it.{max,min}_by(f)` applies `f` to each element and returns the extreme
  - `it.nth(n)` discards `n - 1` elements and returns `n`th
  - `it.reduce(f)` reduces consecutive elements by `x = f(x, y)`
  - `it.skip(n)` returns an iterator with elements of `it` from `n` onward
  - `it.take(n)` returns next `n` values from `it` (consuming it in the process)
  - `it.unzip()` collects tuples into a tuple of containers

### Day 22
- Generic `impl` for generic `struct`: `struct Foo<T> { ... }; impl<T> Foo<T> { ... }`

### Day 23
- Tuple-like structs: `struct Foo(i32, i32, i32);`
- Flush `stdout`: `use std::io::{self, Write}; io::stdout().flush().unwrap();`

### Day 24
- Sometimes it's worth trying solving the problem by hand.