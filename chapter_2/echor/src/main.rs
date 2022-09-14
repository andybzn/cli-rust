use clap::{Arg, Command};

fn main() {

  let matches = Command::new("echor")
      .version("0.1.0")
      .author("Andy Baizon <dev@baizon.uk>")
      .about("Rust echo")
      .arg(
        Arg::with_name("text")
            .value_name("TEXT")
            .help("Input text")
            .required(true)
            .min_values(1),
      )
      .arg(
        Arg::with_name("omit_newline")
            .short('n')
            .help("Do not print newline")
            .takes_value(false),
      )
      .get_matches();

  println!("{:?}", matches);
}
