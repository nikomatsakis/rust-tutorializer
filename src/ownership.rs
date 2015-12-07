fn main() {
    let name = format!("Firefox");
    let name = remove_vowels(name);
    println!("Removing vowels yields {:?}", remove_vowels(name));
    // Goal #2: Convert so that it prints `Removing vowels from
    // "Firefox" yields "Frfx"` instead.
    //
    // Extra bonus: Can you do it without copying any data?
}

fn remove_vowels(name: String) -> String {
    // Goal #1: What is needed here to make this compile?
    // PROMPT let output = String::new();
    // START SOLUTION
    let mut output = String::new();
    // END SOLUTION
    for c in name.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                // skip vowels
            }
            _ => {
                output.push(c);
            }
        }
    }
    output
}
