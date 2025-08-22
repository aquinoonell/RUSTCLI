use clap::{Arg, ArgMatches, command};

fn main() {
    let match_result: ArgMatches = command!()
        .arg(Arg::new("firstname").short('f').long("first-name")
            .required(true).aliases("fname"))
        .arg(Arg::new("lastname").short('l').long("last-name")
            .required(true))
        .get_matches();
}
