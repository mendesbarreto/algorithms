// a = [n]
// fn1(a) => 1 + a [0]
// fn2(a) => sum(a);
// fn3(a) => pair(a);
//

use std::borrow::Borrow;
use std::time::Instant;

// fn1
fn sum_first_element(list: Vec<i32>) -> i32 {
    1 + list[0]
}

// fn2
fn sum_all_items(list: Vec<i32>) -> i32 {
    let mut items = Vec::from(list);

    return match items.into_iter().reduce(|n1, n2| (n1 + n2)) {
        None => 0,
        Some(value) => value,
    };
}

fn pair_list_items(list: Vec<i32>) -> Vec<i32> {
    let mut items = Vec::from(list);
    let items_count = items.len();
    let mut pair_items: Vec<i32> = vec![0; items_count * items_count];

    let mut pair_left: i32 = 0;
    let mut pair_right: i32 = 0;

    for i in 0..(items_count) {
        for j in 0..(items_count) {
            pair_left = items[i];
            pair_right = items[j];

            let formatted_string = format!("{pair_left}{pair_right}");
            pair_items[i * items_count + j] = formatted_string.parse().unwrap();
        }
    }

    return pair_items;
}

fn run_pair_list(list: Vec<i32>) {
    let start = Instant::now();
    pair_list_items(list);
    let duration = start.elapsed();
    println!("Create pairs algo took: {:?}", duration);
}

fn run_first_element(list: Vec<i32>) {
    let start = Instant::now();
    sum_first_element(list);
    let duration = start.elapsed();
    println!("Sum first element  took: {:?}", duration);
}

fn run_sum_all_items(list: Vec<i32>) {
    let start = Instant::now();
    sum_all_items(list);
    let duration = start.elapsed();
    println!("Sum all items: {:?}", duration);
}

fn main() {
    let list = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26,
    ];

    run_pair_list(list.clone());
    run_first_element(list.clone());
    run_sum_all_items(list.clone());
}
