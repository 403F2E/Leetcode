pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let length = nums.len() - 1;
    if length < 2 {
        return vec![vec![]];
    }

    let mut res: Vec<Vec<i32>> = Vec::new();
    res.reserve(length / 2);
    let mut nums = nums;
    nums.sort();

    let mut sum: i32;

    for i in 0..length - 1 {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        if nums[i] > 0 {
            return res;
        }

        let (mut j, mut k) = (i + 1, length);
        while j < k {
            sum = nums[i] + nums[j] + nums[k];
            if sum > 0 {
                k -= 1;
            } else if sum < 0 {
                j += 1;
            } else {
                res.push(vec![nums[i], nums[j], nums[k]]);
                j += 1;
                while nums[j] == nums[j - 1] && j < k {
                    j += 1;
                }
            }
        }
    }

    res
}

fn main() {
    let res = three_sum(vec![-1, 0, 1, 2, -1, -4]);
    println!("{:?}", res);
}
