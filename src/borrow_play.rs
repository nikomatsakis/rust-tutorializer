fn main() {
    let name = format!("fellow Rustaceans");
    helper(&name);
    helper(&name);
}

fn helper(name: &String) {
    println!("Hello, {}!", name);
}
