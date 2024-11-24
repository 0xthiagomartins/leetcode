struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        // Mapping of Roman numerals to their integer values
        let roman_values = [
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
        .iter()
        .cloned()
        .collect::<std::collections::HashMap<_, _>>();

        let mut total = 0;
        let mut prev_value = 0;

        // Iterate over each character in the string
        for c in s.chars() {
            // Get the current value from the mapping
            let current_value = *roman_values.get(&c).unwrap();

            // If the current value is greater than the previous value,
            // we need to subtract twice the previous value (since it was added before)
            if current_value > prev_value {
                total += current_value - 2 * prev_value;
            } else {
                total += current_value;
            }

            // Update previous value for next iteration
            prev_value = current_value;
        }

        total
    }
}

fn main() {
    let result1 = Solution::roman_to_int("III".to_string());
    println!("III -> {}", result1); // Output: 3

    let result2 = Solution::roman_to_int("LVIII".to_string());
    println!("LVIII -> {}", result2); // Output: 58

    let result3 = Solution::roman_to_int("MCMXCIV".to_string());
    println!("MCMXCIV -> {}", result3); // Output: 1994
}
