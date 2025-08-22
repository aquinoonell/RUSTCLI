use clap::{Arg, ArgAction, ArgMatches, Command, arg, command};

fn main() {
    let match_result: ArgMatches = command!()
        .arg(
            Arg::new("firstname")
                .short('f')
                .long("first-name")
                .required(true)
                .alias("fname"),
        )
        .arg(
            Arg::new("lastname")
                .short('l')
                .long("last-name")
                .alias("lname")
                .required(true),
        )
        .subcommand(
            Command::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "list test values").action(ArgAction::SetTrue)),
        )
        .get_matches();
}
