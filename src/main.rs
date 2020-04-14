use std::env;
use std::collections::HashMap;

fn main() {
    let mut args = env::args();

    let n = args.len() - 1;

    // this will be populated with the arguments, sorted in ascending order
    let mut elements: Vec<i32> = Vec::with_capacity(n);

    let mut sum = 0;

    let mut counts: HashMap<i32, i32> = HashMap::new();
    let mut mode: (Vec<i32>, i32) = (vec![], 0);

    let mut median = 0f64;
    let last_median_index = if n % 2 == 0 {
        n / 2 + 1
    } else {
        n / 2
    };

    args.next(); // the first argument is the path to the executable, we can discard it
    for i in 0..n {
        let x = args.next().unwrap();
        let x: i32 = match x.parse() {
            Ok(x) => x,
            _ => {
                println!("{} is not an integer. It will be skipped.", x);
                continue;
            }
        };

        sum += x;

        let mut num_times_seen = *counts.entry(x).or_insert(0);
        num_times_seen += 1;

        if num_times_seen > mode.1 {
            mode = (vec![x], num_times_seen);
        } else if num_times_seen == mode.1 {
            mode.0.push(x);
        }

        if i <= last_median_index {
            // run an ascending insertion sort on the data as it comes in
            elements.push(x);
            let mut cursor = i; // this points towards the edge of the sorted data
            while cursor > 0 && x <= elements[cursor - 1] {
                cursor -= 1;
                elements[cursor + 1] = elements[cursor];
                elements[cursor] = x;
            }
        }

        // find out where the median is
        if n % 2 == 0 && (i == n / 2 - 1 || i == n / 2) {
            // if there are an even amount of elements, average the two in the center
            median += *elements.last().unwrap() as f64 * 0.5;
        } else if i == n / 2 {
            median = *elements.last().unwrap() as f64;
        }
    }

    if n > 0 {
        println!("Mean: {}", sum as f64 / n as f64);
        println!("Mode: {:?}", mode.0);
        println!("Median: {}", median);
    } else {
        println!("Usage: enter a list of numbers as arguments to the program");
    }
}
