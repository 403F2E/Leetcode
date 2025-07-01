pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<(usize, i32)> = Vec::new();
    let mut res = vec![0; temperatures.len()];

    for i in 0..temperatures.len() {
        while !stack.is_empty() && stack[stack.len() - 1].1 < temperatures[i] {
            let (prev, _) = stack.pop().unwrap();
            res[prev] = (i - prev) as i32;
        }
        stack.push((i, temperatures[i]));
    }

    res
}
