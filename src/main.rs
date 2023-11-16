use clap::{App, Arg};
use std::io::stdin;
use urlencoding::{decode, encode};

fn main() {
    let decode_arg = Arg::new("decode")
        .short('d')
        .long("decode")
        .help("decode data");

    let file_arg = Arg::new("FILE").index(1).required(false);

    let args = App::new("urlencode")
        .about("")
        .arg(decode_arg)
        .arg(file_arg)
        .get_matches();

    let mut input = String::new();
    stdin().read_line(&mut input).expect("No input provided");

    if args.is_present("decode") {
        println!("{}", decode(&input.trim_end()).unwrap());
        return;
    }
    println!("{}", encode(&input.trim_end()));
}
