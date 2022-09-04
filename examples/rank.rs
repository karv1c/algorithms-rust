/* Given an array of integers arr, replace each element with its rank.

The rank represents how large the element is. The rank has the following rules:

Rank is an integer starting from 1.
The larger the element, the larger the rank. If two elements are equal, their rank must be the same.
Rank should be as small as possible. */

use std::collections::HashMap;

pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    let mut rvec = Vec::<i32>::new();
    let mut arr_clone = arr.clone();
    arr_clone.sort();
    arr_clone.dedup();
    let mut map = HashMap::<i32, i32>::new();
    for i in 0..arr_clone.len() {
        map.insert(arr_clone[i], i as i32);
    }
    for i in 0..arr.len() {
        rvec.push(*map.get(&arr[i]).unwrap() + 1)
    }
    rvec
}

fn main() {
    let arr = vec![40, 10, 20, 30, 40];
    println!("{:?}", array_rank_transform(arr))
}
