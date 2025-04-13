/// <h1>Two Sum II - Input Array Is Sorted</h1>
/// Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order,
/// find two numbers such that they add up to a specific target number. 
/// Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 <= numbers.length.
/// Return the indices of the two numbers, index1 and index2, added by one as an integer array [index1, index2] of length 2.

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let (mut l, mut r) = (0, numbers.len() - 1) ;
    loop {
        if numbers[l] + numbers[r] > target {r -= 1;}
        else if numbers[l] + numbers[r] < target {l += 1}
        else {
            break;
        }
    }
    vec![(l + 1) as i32, (r + 1) as i32]
}
