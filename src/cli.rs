pub fn run() {
    let mut hello = String::from("Hello");

    hello.push_str(" world");

    if hello.is_empty() {
        println!("String is empty!")
    } else {
        println!("String is not empty")
    }

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    println!("{}", hello.len());
    println!("{}", hello);
    println!("Capacity: {}", hello.capacity());
}