//1.Implement a function that checks whether a given string is a palindrome or not.


fn is_palindrome(s: &str) -> bool {
    let reversed = s.chars().rev().collect::<String>();
    s == reversed
}

fn main() {
    let input = "radar";
    if is_palindrome(input) {
        println!("{} is a palindrome.", input);
    } else {
        println!("{} is not a palindrome.", input);
    }
}
