use std::io;

pub(crate) fn read_buffer(message: String) -> String {
    println!("> {}", message);

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    return buffer.trim().to_string();
}
