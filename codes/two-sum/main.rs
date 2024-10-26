use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut lookup = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let diff = target - num;
            if let Some(&index) = lookup.get(&diff) {
                return vec![index as i32, i as i32];
            }
            lookup.insert(num, i);
        }
        vec![]
    }
}

fn main() {
    // Test case 1
    let nums1 = vec![2, 7, 11, 15];
    let target1 = 9;
    assert_eq!(Solution::two_sum(nums1, target1), vec![0, 1]);

    // Test case 2
    let nums2 = vec![3, 2, 4];
    let target2 = 6;
    assert_eq!(Solution::two_sum(nums2, target2), vec![1, 2]);

    // Test case 3
    let nums3 = vec![3, 3];
    let target3 = 6;
    assert_eq!(Solution::two_sum(nums3, target3), vec![0, 1]);
}
