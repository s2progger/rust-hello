fn main() {
    let name = "Rust".to_string();

    say_hello(&name);
}

fn say_hello(name: &String) {
    println!("Hello, {}!", name);
}
