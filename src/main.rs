use urlencoding::encode;

fn main() {
    let mut param = std::env::args().skip(1);

    let message = if let Some(content) = param.next() {
        match content.trim() {
            "" => {
                eprintln!("Please inform non-empty content");
                String::new()
            }
            trimmed_content => trimmed_content.to_string(),
        }
    } else {
        eprintln!("Please inform the content");
        String::new()
    };

    println!("{}", encode(&message));
}
