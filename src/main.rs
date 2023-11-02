mod enigma;

use clap::{App, Arg};

fn main() {
  // Define an matches varible for clap app gateway. 
    let matches = App::new("Enigma Simulator")
      // Define "dial" function.
      // Dial function will emulate the enigma dial cog part.
      // Dial function will return a vector of dial positions.
      // TODO: Expand the dial from 100 digits to unlimited digits.
      // MAYBE? Add new function "max-dial" to return the maximum dial position.
        .arg(
            Arg::with_name("dial") // What's the name of this argument?
                .short('d') // The short name or alias of the function
                .long("dial") // The long name of the function
                .multiple(true) // Can this argument repeat?
                .takes_value(true) // Does this argument need a follow-up value?
                .help("Set Enigma dial positions"), // The help mesage of this function
        )
      // Define "wire" function.
      // Wire function will emulate the enigma wire part.
      // Wire function will return a vector of (from, to) pairs.
      // TODO: More flexable wire connection, like 3-dots wiring, 4-dots wiring, etc..
        .arg(
            Arg::with_name("wire")
                .short('w')
                .long("wire")
                .multiple(true)
                .takes_value(true)
                .help("Set wire configurations"),
        )
      // Setup an argument for input string.
        .arg(
          Arg::with_name("INPUT")
          .help("What string you want to encrpyt?")
          .required(true) // Is this argument required?
        )
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
