use std::cmp::{max, min};

fn max_area(height: Vec<i32>) -> i32 {
    let mut n = (height.len() - 1) as i32;
    let mut max_area = 0i32;

    if n == 1 {
        return min(height[0], height[1]);
    }

    let (mut l, mut h) = (0usize, n as usize);

    while l < h {
        let min_poll = min(height[l], height[h]);
        let area = min_poll * n;

        max_area = max(max_area, area);

        if height[l] < height[h] {
            l += 1;
        } else if height[l] >= height[h] {
            h -= 1;
        }
        n -= 1;
    }
    max_area
}

fn main() {
    let example1: Vec<i32> = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let example2: Vec<i32> = vec![1, 1];

    println!("{}", max_area(example1));
    println!("{}", max_area(example2));
}
