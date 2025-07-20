// Given an array of integers heights representing the histogram's bar height where the width of each bar is 1.
// return the area of the largest rectangle in the histogram.

use std::cmp::max;

fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut stack: Vec<(usize, i32)> = Vec::new();
    let mut res = 0i32;

    for (i, height) in heights.iter().enumerate() {
        let mut start = i;
        while !stack.is_empty() && stack[stack.len() - 1].1 > *height {
            let top = stack.pop().unwrap();
            res = max(res, top.1 * (i - top.0) as i32);
            start = top.0;
        }
        stack.push((start, *height));
    }

    for (i, h) in &stack {
        res = max(res, h * (heights.len() - i) as i32);
    }

    res
}

fn main() {
    let example1 = vec![2, 1, 5, 6, 2, 3];
    let example2 = vec![2, 4];

    let res1 = largest_rectangle_area(example1);
    let res2 = largest_rectangle_area(example2);

    println!("result1: {}, result2: {}", res1, res2);
}
