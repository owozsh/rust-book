fn main() {
    let my_string = String::from("Hello, World");

    let word = first_word(&my_string);
    println!("{}", word);

    let word = first_word(&my_string[..3]);
    println!("{}", word);

    let word = first_word(&my_string[7..]);
    println!("{}", word);
}

fn first_word(string: &str) -> &str {
    let bytes = string.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &string[..index];
        }
    }

    &string[..]
}
