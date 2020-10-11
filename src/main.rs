use std::collections::HashMap;

fn main() {
    let numbers = vec![5, 2, 2, 2, 2];

    println!("The average is: {}", average(&numbers));
    println!("The median is: {}", median(&numbers));
    println!("The dominant is: {}", dominant(&numbers));
}

fn average(numbers: &Vec<i32>) -> i32 {
    let mut sum = 0;

    for num in numbers {
        sum += num;
    }

    sum / numbers.len() as i32
}

fn median(numbers: &Vec<i32>) -> f64 {
    let mut numbers = numbers.clone();
    numbers.sort();

    let len = numbers.len();

    if len < 1 {
        return 0.0;
    } else if len % 2 == 1 {
        return *numbers.get(len / 2).unwrap() as f64;
    }

    (*numbers.get(len / 2 - 1).unwrap() + *numbers.get(len / 2).unwrap()) as f64 / 2.0
}

fn dominant(numbers: &Vec<i32>) -> i32 {
    let mut most_popular = HashMap::new();

    for num in numbers {
        let count = most_popular.entry(num.to_string()).or_insert(0);
        *count += 1;
    }

    let (dominant, _) = most_popular.iter().max_by_key(|entry| entry.1).unwrap();
    let dominant: i32 = dominant.parse().expect("xD");

    dominant
}