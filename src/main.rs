use urlencoding::encode;

fn main() {
    let params: Vec<String> = std::env::args().skip(1).collect();

    if params.is_empty() {
        eprintln!("Please inform at least one parameter");
        return;
    }

    let message = params.join(" ");

    println!("{}", encode(&message));
}
