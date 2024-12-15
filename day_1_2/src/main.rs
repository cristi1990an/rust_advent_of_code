// https://adventofcode.com/2024/day/1

fn read_input() -> (Vec<u32>, Vec<u32>) {
    use std::fs::File;
    use std::io::{self, BufRead};

    let mut result: (Vec<u32>, Vec<u32>) = (vec![], vec![]);

    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let (first, second) = line.split_once(' ').unwrap();
        let lhs = first.trim().parse::<u32>().unwrap();
        let rhs = second.trim().parse::<u32>().unwrap();
        println!("Parsed ({lhs}, {rhs})");

        result.0.push(lhs);
        result.1.push(rhs);
    }

    return result;
}

fn main() {
    use std::collections::HashMap;

    let (col_1, col_2) = read_input();

    // We're going to create a HashMap (std::unordered_map) to count
    // the number of occurences of each value in the second column.
    let mut counter: HashMap<u32, u32> = HashMap::new();

    for value in col_2 {
        // HashMap has a complex yet intuitive API for inserting and extracting key-value pairs.
        // Our use case it to insert 0 as the associated value of a key if said key wasn't inserted
        // before or increment said value if it was. Let's look at how we can do this:
        // .entry()      - returns an abstraction over the value associated with a given key,
        //                 value which might not exist yet.
        // .or_insert()  - will return the value or, if it doesn't exist, will insert a specified
        //                 value and return a reference to the existing or newly inserted value
        // .and_modify() - will apply a mutation on the value, only if said value already exists
        //                 and then it will return a reference to the entry again for futher chaining.
        //
        // These are the methods we'll be using.
        counter
            .entry(value)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    // Let's define a helping closure that either returns the counter associated with a key
    // from the map or 0 as a default (without inserting it into the map).
    let get_count = |val| counter.get(val).unwrap_or(&0);

    // Multiply each value in col_1 with its counter from the map and sum all the value.
    let sum = col_1.iter().map(|val| val * get_count(val)).sum::<u32>();

    println!("Sum = {sum}");
}
