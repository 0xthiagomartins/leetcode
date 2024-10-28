struct TwoSum;

impl TwoSum {
    fn brute_force(&self, nums: &[i32], target: i32) -> Vec<usize> {
        for (i1, &num1) in nums.iter().enumerate() {
            for (i2, &num2) in nums.iter().enumerate() {
                if num1 + num2 == target && i1 != i2 {
                    return vec![i1, i2];
                }
            }
        }
        vec![]
    }

    fn two_pass_hash_table(&self, nums: &[i32], target: i32) -> Vec<usize> {
        use std::collections::HashMap;
        let mut lookup = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            lookup.insert(num, i);
        }
        for (i, &num) in nums.iter().enumerate() {
            let diff = target - num;
            if let Some(&j) = lookup.get(&diff) {
                if i != j {
                    return vec![i, j];
                }
            }
        }
        vec![]
    }

    fn one_pass_hash_table(&self, nums: &[i32], target: i32) -> Vec<usize> {
        use std::collections::HashMap;
        let mut lookup = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let diff = target - num;
            if let Some(&j) = lookup.get(&diff) {
                return vec![j, i];
            }
            lookup.insert(num, i);
        }
        vec![]
    }
}

fn main() {
    let solution = TwoSum;

    let approaches: Vec<fn(&TwoSum, &[i32], i32) -> Vec<usize>> = vec![
        TwoSum::brute_force,
        TwoSum::two_pass_hash_table,
        TwoSum::one_pass_hash_table,
    ];

    for approach in approaches {
        // Since Rust doesn't have a direct way to get the function name, we'll match manually
        let approach_name = match approach as usize {
            x if x == (TwoSum::brute_force as usize) => "brute_force",
            x if x == (TwoSum::two_pass_hash_table as usize) => "two_pass_hash_table",
            x if x == (TwoSum::one_pass_hash_table as usize) => "one_pass_hash_table",
            _ => "unknown",
        };
        println!("=== Testing approach {} ===", approach_name);

        // Test case 1
        let nums1 = vec![2, 7, 11, 15];
        let target1 = 9;
        assert_eq!(approach(&solution, &nums1, target1), vec![0, 1]);
        println!("Test case 1 passed");

        // Test case 2
        let nums2 = vec![3, 2, 4];
        let target2 = 6;
        assert_eq!(approach(&solution, &nums2, target2), vec![1, 2]);
        println!("Test case 2 passed");

        // Test case 3
        let nums3 = vec![3, 3];
        let target3 = 6;
        assert_eq!(approach(&solution, &nums3, target3), vec![0, 1]);
        println!("Test case 3 passed\n");
    }
}
