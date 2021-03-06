use std::env;
use std::process;

use formulae::Input;

fn main() {
  let args: Vec<String> = env::args().collect();

  let input = Input::new_from_args(&args).unwrap_or_else(|err| {
    println!("Error when collecting input: {}", err);
    process::exit(1);
  });

  println!("{:#?}", input);
}
