/* Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
You may assume that each input would have exactly one solution, and you may not use the same element twice.
You can return the answer in any order. */

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut out = HashMap::<i32, i32>::new();
    for i in 0..nums.len() {
        let sub = target - nums[i as usize];
        if out.contains_key(&sub) {
            return vec![i as i32, *out.get(&sub).unwrap()];
        } else {
            out.insert(nums[i as usize], i as i32);
        }
    }
    return Vec::<i32>::new();
}

fn main() {
    let nums = vec![2, 7, 11, 13];
    let target = 9;
    println!("{:?}", two_sum(nums, target));
}
