struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut vec_of_chars_of_x: Vec<char> = Vec::new();
        let x_to_string = x.to_string();

        let mut number_of_index = x_to_string.len();

        for char in x_to_string.chars() {
            println!("{}", char);

            vec_of_chars_of_x.insert(number_of_index, char);

            number_of_index -= 1;
        }

        println!("Vector: {:?}", vec_of_chars_of_x);

        true
    }
}

fn main() {
    Solution::is_palindrome(123);
}
