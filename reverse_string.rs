//9.Reverse a string in Rust
fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    let input = "Hello, world!";
    let reversed = reverse_string(input);
    println!("Original: {}", input);
    println!("Reversed: {}", reversed);
}
