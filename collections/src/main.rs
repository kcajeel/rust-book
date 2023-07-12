/*
Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
and mode (the value that occurs most often; a hash map will be helpful here) of the list.
 */
use std::collections::HashMap;

fn main() {
    let mut arr = [1, 1, 2, 7, 3, 4, 5, 6, 3, 4];
    let mode = get_mode(&arr);
    match mode {
        Some(..) => println!("The mode is {}.",mode.unwrap_or(0)),
        None => println!("No numbers to find the mode of"),
    }
    //right answer is 3
    println!("The median is {}.",get_median(&mut arr));
    //right answer is 4
}

//this function only works if there is a mode
fn get_mode(arr: &[i32]) -> Option<i32> {
    // let mut max = 0;
    // for num in arr.iter().enumerate() {
    //     let mut count = 0;
    //     for value in arr.iter() {
    //         if value == num.1 {
    //             count += 1;
    //         }
    //         if count > max {
    //             max = count;
    //         }
    //     }
    // }
    // max
    let mut counts = HashMap::new();
    arr.iter().copied().max_by_key(|&n| {
        let count = counts.entry(n).or_insert(0);
        *count += 1;
        *count
    })
}

fn get_median(arr: &mut [i32]) -> i32 {
    let new_arr: &mut[i32] = arr;
    new_arr.sort();
    if new_arr.len() %2 == 0 {
        (new_arr[new_arr.len() / 2] + new_arr[new_arr.len() / 2 + 1]) / 2
    }
    else {
        new_arr[(new_arr.len() + 1) / 2 - 1]
    }
}
