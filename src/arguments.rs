use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Fch")]
#[command(author = "Zi Hao Liang <zihaoliang0413@gmail.com>")]
#[command(version = "0.2.0")]
#[command(about="Layer lookup utility for translating between CAD abbreviated layers and layman.", long_about = None)]
pub struct Args {
    /// Switch to building a new dictionary.
    // #[arg(short, long, default_value_t = false)]
    // pub build: bool,

    // /// Switch to finding the layman definition for a given abbreviation sequence.
    // #[arg(short, long, default_value_t = false)]
    // pub short: bool,
    /// The text to be translated.
    #[arg(required = true)]
    pub dictionary: Vec<String>,
}

// /// Name of the person to greet
// #[arg(short, long, default_value_t={"0".to_string()})]
// name: String,
