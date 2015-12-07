pub fn main() {
    let str1 = format!("fellow ");
    let str2 = format!("Rustaceans");
    let str3 = strcat(str1, str2);
    println!("{}", str3);
}

/// Concatenate `suffix` onto the end of `prefix`.
fn strcat(mut prefix: String, suffix: String) -> String {
    for ch in suffix.chars() {
        prefix.push(ch);
    }
    prefix
}

// Challenge: Convert `strcat` to use borrowing, not ownership.

// Question: Now that you've converted `strcat`, what happens if you
// call `strcat` using the same string for `prefix` and `suffix`?
// Why?
