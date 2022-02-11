// a = [n]
// fn1(a) => 1 + a [0]
// fn2(a) => sum(a);
// fn3(a) => pair(a);
//

use std::borrow::Borrow;

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

fn main() {
    let list = vec![1, 2, 3, 4, 5];
    let pair_list = pair_list_items(list);
    println!("Hello, world! {:?}", pair_list);
}
