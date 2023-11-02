mod enigma;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Enigma Simulator")
        .arg(
            Arg::with_name("dial")
                .short("d")
                .long("dial")
                .multiple(true)
                .takes_value(true)
                .help("Set dial positions"),
        )
        .arg(
            Arg::with_name("wire")
                .short("w")
                .long("wire")
                .multiple(true)
                .takes_value(true)
                .help("Set wire configurations"),
        )
        .arg(Arg::with_name("INPUT").help("Input string").required(true))
        .get_matches();

    let dials: Vec<u32> = matches
        .values_of("dial")
        .unwrap_or_default()
        .filter_map(|s| s.parse().ok())
        .collect();

    let wires: Vec<(char, char)> = matches
        .values_of("wire")
        .unwrap_or_default()
        .filter_map(|s| {
            if s.len() == 2 {
                Some((s.chars().next().unwrap(), s.chars().nth(1).unwrap()))
            } else {
                None
            }
        })
        .collect();

    let input = matches.value_of("INPUT").unwrap();

    let enigma = Enigma { dials, wires };
    let encrypted = enigma.encrypt(input);

    println!("{}", encrypted);
}
