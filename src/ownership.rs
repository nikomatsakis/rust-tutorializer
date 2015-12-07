fn main() {
    let name = format!("Firefox");
    print_out(name);
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

fn print_out(name: String) -> String {
    let devowelized_name = remove_vowels(name);
    println!("Removing vowels yields {:?}", devowelized_name);

    // Goal #2: What happens when you uncomment the following
    // line? Can you change the code above so that this next line?
    // println!("Removing vowels from {:?} yields {:?}",
    //          name, devowelized_name);

    // Extra credit: Can you do it without copying any data?
    // (Using only ownership transfer)
}
