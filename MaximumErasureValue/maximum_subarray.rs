// You are given an array of positive integers nums and want to erase a subarray containing unique elements. The score you get by erasing the subarray is equal to the sum of its elements.

// Return the maximum score you can get by erasing exactly one subarray.

// An array b is called to be a subarray of a if it forms a contiguous subsequence of a, that is, if it is equal to a[l],a[l+1],...,a[r] for some (l,r).

pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
    let (mut max, mut score) = (0i32, 0i32);
    let (mut i, mut j, end) = (0usize, 0usize, nums.len());
    let mut unique: [bool; 10001] = [false; 10001];

    while j < end {
        if unique[nums[j] as usize] {
            max = max.max(score);

            while nums[i] != nums[j] {
                score -= nums[i];
                unique[nums[i] as usize] = false;
                i += 1;
            }
            i += 1;
        } else {
            score += nums[j];
            unique[nums[j] as usize] = true;
        }
        j += 1;
    }

    max = max.max(score);

    max
}

fn main() {
    let (example1, example2) = ([4, 2, 4, 5, 6], [5, 2, 1, 2, 5, 2, 1, 2, 5]);

    println!("{}", maximum_unique_subarray(example1));
    println!("{}", maximum_unique_subarray(example2));
}
