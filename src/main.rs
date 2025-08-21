use clap::{Arg, ArgMatches, command};

fn main() {
    let match_result: ArgMatches = command!()
        .arg(Arg::new("firstname").short('f'))
        .arg(Arg::new("lastname").short('l'))
        .get_matches();
}
