// Takes in a text string from the CLI and returns the spongebobified version.

use clap::{App, Arg};
use rand::Rng;

fn main() {
    // Get CLI arguments
    let matches = get_matches();

    let text = matches.value_of("text").unwrap().to_lowercase();
    let mut chance = matches
        .value_of("chance")
        .unwrap_or("0.5")
        .parse::<f64>()
        .unwrap();

    // If chance is more than 1, assume the user meant that in percentage
    if chance > 1.0 {
        chance /= 100.0;
    }
    if chance < 0.0 {
        chance = 0.0;
    }

    let mut spongebobified = String::new();

    // Spongebobify!
    for c in text.chars() {
        let mut rng = rand::thread_rng();

        if c.is_alphabetic() && rng.gen_bool(f64::from(chance)) {
            spongebobified.push(c.to_ascii_uppercase());
        } else {
            spongebobified.push(c);
        }
    }

    println!("{}", spongebobified);
}

fn get_matches() -> clap::ArgMatches<'static> {
    App::new("Spongebobify")
        .version("1.2.2")
        .author("Oliver A. Thunæs <oliver@netron.no>")
        .about("Spongebobify your text")
        .arg(
            Arg::with_name("text")
                .help("Text to spongebobify")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("chance")
                .short("c")
                .long("chance")
                .takes_value(true)
                .help("Chance of a character being spongebobified"),
        )
        .get_matches()
}
