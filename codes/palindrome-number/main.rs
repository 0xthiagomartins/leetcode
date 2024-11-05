impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut original = x;
        let mut reversed = 0;

        while original != 0 {
            let digit = original % 10;
            // Prevent overflow
            if reversed > (i32::MAX - digit) / 10 {
                return false;
            }
            reversed = reversed * 10 + digit;
            original /= 10;
        }

        reversed == x
    }
}

fn main() {
    let test_cases = vec![
        (121, true),
        (-121, false),
        (10, false),
        (0, true),
        (12321, true),
        (123, false),
    ];

    for (input, expected) in test_cases {
        assert_eq!(Solution::is_palindrome(input), expected);
        println!("Test case with input {} passed", input);
    }
}
