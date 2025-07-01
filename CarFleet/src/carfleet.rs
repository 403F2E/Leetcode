/// There are n cars at given miles away from the starting mile 0, traveling to reach the mile target.

/// You are given two integer array position and speed, both of length n, where position[i] is the starting mile of the ith car and speed[i] is the speed of the ith car in miles per hour.
//
/// A car cannot pass another car, but it can catch up and then travel next to it at the speed of the slower car.
//
/// A car fleet is a car or cars driving next to each other. The speed of the car fleet is the minimum speed of any car in the fleet.
//
/// If a car catches up to a car fleet at the mile target, it will still be considered as part of the car fleet.
//
/// Return the number of car fleets that will arrive at the destination.

pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    if position.len() == 1 {
        return 1;
    }

    let target = target as f32;
    let mut pair: Vec<(i32, i32)> = Vec::new();
    for i in 0..position.len() {
        pair.push((position[i], speed[i]));
    }
    pair.sort();

    let mut stack: Vec<f32> = Vec::new();
    for i in (0..pair.len()).rev() {
        let p = pair[i].0 as f32;
        let s = pair[i].1 as f32;
        stack.push((target - p) / s);
        if stack.len() >= 2 && stack[stack.len() - 1] <= stack[stack.len() - 2] {
            stack.pop();
        }
    }

    (stack.len() as i32)
}
