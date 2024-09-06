struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x_to_string: String = x.to_string();
        let mut  y_string: String = String::new();

        for char in x_to_string.chars().rev() {
            y_string.push(char);
        }

        if x_to_string == y_string {
            true
        } else {
            false
        }
    }
}

fn main() {
    let test = Solution::is_palindrome(123);
    println!("{:?}", test);
}
