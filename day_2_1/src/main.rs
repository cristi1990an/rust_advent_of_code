fn read_input() -> Vec<Vec<u32>> {
    use std::fs::File;
    use std::io::{self, BufRead};

    let mut result: Vec<Vec<u32>> = vec![];

    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);

    for line_result in reader.lines() {
        result.push(vec![]);

        let line = line_result.unwrap();

        // Split a string into slices delimited by whitespaces
        for word in line.split_whitespace() {
            let last_vec = result.last_mut().unwrap();
            last_vec.push(word.parse::<u32>().unwrap());
        }
    }

    return result;
}

fn is_safe_report(report: Vec<u32>) -> bool {
    print!("{:?}: ", report);

    if !report.is_sorted() && !report.is_sorted_by(|lhs, rhs| lhs > rhs) {
        println!("Unsafe");
        return false;
    }
    let slider = report.windows(2);

    let mut diffs = slider.map(|window| window[0].abs_diff(window[1]));

    if diffs.any(|diff| diff < 1 || diff > 3) {
        println!("Unsafe");
        return false;
    }

    println!("Safe");
    return true;
};

fn main() {
    let input = read_input();

    let is_safe = |report: &&Vec<u32>| -> bool {
        print!("{:?}: ", report);
        if !report.is_sorted() && !report.is_sorted_by(|lhs, rhs| lhs > rhs) {
            println!("Unsafe");
            return false;
        }
        let slider = report.windows(2);
        let mut diffs = slider.map(|window| window[0].abs_diff(window[1]));

        if diffs.any(|diff| diff < 1 || diff > 3) {
            println!("Unsafe");
            return false;
        }

        println!("Safe");
        return true;
    };

    // Apply filter with the closure defined above and then then call .count().
    // The difference between .len() and .count() is that the length of a collection is known in constant time,
    // while count requires to iterate through the collection.
    let safe_count = input.iter().filter(is_safe).count();

    println!("Safe count: {safe_count}");
}
