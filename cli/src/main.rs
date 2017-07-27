extern crate docopt;
extern crate my_password;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::{env, process};
use docopt::{Docopt, Error};

pub const USAGE: &'static str = r#"
my password manager
    Copyright 2017 Marek Kotewicz

Usage:
    my-password <password> <email> <service>
    my-password -h | --help

Options:
    -h, --help         Display this message and exit.
"#;


#[derive(Debug, Deserialize)]
struct Args {
	arg_password: String,
	arg_email: String,
	arg_service: String,
}

fn main() {
	match execute(env::args()) {
		Ok(result) => println!("{}", result),
		Err(err) => {
			println!("{}", err);
			process::exit(1);
		}
	}
}

fn execute<S, I>(command: I) -> Result<String, Error> where I: IntoIterator<Item=S>, S: AsRef<str> {
	let args: Args = Docopt::new(USAGE)
		.and_then(|d| d.argv(command).deserialize())?;

	Ok(my_password::password(&args.arg_password, &args.arg_email, &args.arg_service))
}
