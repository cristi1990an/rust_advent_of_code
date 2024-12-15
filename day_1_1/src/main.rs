// https://adventofcode.com/2024/day/1

// reads and parses inputs from a file
// returns a tuple of two vectors
fn read_input() -> (Vec<u32>, Vec<u32>) {
    // equivalent of "using namespace std::_" from C+=
    use std::fs::File;
    use std::io::{self, BufRead};

    let mut result: (Vec<u32>, Vec<u32>) = (vec![], vec![]);

    // Opens and creates file handles
    let file = File::open("input.txt").unwrap();
    // Wraps handles in a buffer reader helper class
    let reader = io::BufReader::new(file);

    // BufReader has lines() method that generates a string in each iteration containing a read line
    for line_result in reader.lines() {
        // For some reason, each string is wrapped into a Result<String, Error>
        // Result<_, _> is the equivalent of std::expected<_, _>, a construct that can either
        // hold a value or an error. Pretty safe to simply unwrap in this case.
        let line = line_result.unwrap();

        // A String in Rust has a "split_once(delim)" method which returns two slices of type &str.
        // String slices are like std::string_view's but are a built-in language feature in Rust.
        // Like most things in Rust this result is wrapped into an Option<_> (std::optional) in case it fails.
        // Since we know the input format, it's safe to unwrap and get the value.
        let (first, second) = line.split_once(' ').unwrap();

        // We call trim() on each to eliminate any leading or trailing whitespaces and parse::<u32> to
        // convert the string into an u32. Result is unwrapped since this shouldn't fail.
        let lhs = first.trim().parse::<u32>().unwrap();
        let rhs = second.trim().parse::<u32>().unwrap();

        // For debug, we're printing what we're reading.
        println!("Parsed ({lhs}, {rhs})");

        // Elements of a tuple in Rust can be accessed through tup.INDEX.
        // The equivalent in C++ would be "std::get<INDEX>(my_tuple)".
        result.0.push(lhs); // <- push to first vector
        result.1.push(rhs); // <- push to second vector
    }

    return result;
}

fn main() {
    // When saving the result of the function call, we can directly destructure the tuple.
    // In C++ the equivalent would be "auto [col_1, col_2] = read_input()".
    let (mut col_1, mut col_2) = read_input();

    // Sort each vector
    col_1.sort();
    col_2.sort();

    // Zip creates a range of tuples between each adjacent element in each vector.
    // aka: [ (col_1[0], col_2[0]), (col_1[1], col_2[1]), ...]
    let pairs = std::iter::zip(col_1, col_2);

    // Map makes a transformation on each element, in this case we transform each tuple into
    // the absolute difference of its two elements.
    let diffs = pairs.map(|(lhs, rhs)| lhs.abs_diff(rhs));

    // Each iterable object has the .sum() method that simply consumes the range and sums up the elements.
    let sum = diffs.sum::<u32>();

    // Answer
    println!("Sum = {sum}");

    /*
       NOTES:

       Passing col_1 and col_2 into the zip(...) method will make the resulting iterable gain ownership of both.
       Reffering to either vector after line 61 will result in a compiler error since the ownership of these
       two identifier was lost. If we want to pass ranges into algorithms without having said algorithm become
       the owner, we must first create non-owning iterable proxies using the .iter() method.

       let pairs = std::iter::zip(col_1.iter(), col_2.iter());

       Now, the closure (lambda) method at line 65 must also be changed, to understand why, we must first see its more verbose
       signature. Take this equivalent closure for example:

       |(lhs, rhs) : (u32, u32) | lhs.abs_diff(rhs)

       This is the same as the one wrote at line 65, the only difference being that here we're explicit about the types
       of the tuple, while at line 65 the compiler was deducing them.

       After passing the vectors through the .iter() proxies, the signarure becomes:

       |(lhs, rhs) : (&u32, &u32) | <- references to u32 as opposed to owning values

       The full closure definition must become:

       |(lhs, rhs)| lhs.abs_diff(*rhs)

       Since lhs/rhs are references to u32, to pass them into methods that expect u32 we must dereference them. Calling
       a method of the pointed-to object ('abs_diff' in this case) doesn't require the dereference *.
    */
}
