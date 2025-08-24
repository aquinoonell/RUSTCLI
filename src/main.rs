use clap::{Arg, ArgAction, ArgMatches, Command, arg, command};

fn main() {
    let matches = command!()
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
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .subcommand(
            Command::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "list test values").action(ArgAction::SetTrue)),
        )
        .get_matches();

    match matches
        .get_one::<u8>("debug")
        .expect("Counts are defaulted")
    {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.get_flag("list") {
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists..");
        }
    }
}
