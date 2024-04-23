//6.Implement a function that finds the longest common prefix of a given set of strings

fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut prefix = String::new();
    let first_str = &strs[0];

    for (i, ch) in first_str.chars().enumerate() {
        for s in strs.iter().skip(1) {
            if i >= s.len() || s.chars().nth(i) != Some(ch) {
                return prefix;
            }
        }
        prefix.push(ch);
    }

    prefix
}

fn main() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    let common_prefix = longest_common_prefix(&strings);
    println!("Longest common prefix: {}", common_prefix);
}

