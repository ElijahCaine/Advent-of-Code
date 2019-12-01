extern crate clap;

mod one;

use clap::{
    Arg,
    App,
    SubCommand,
};

fn main() {
    let matches = App::new("aoc2019")
        .about("Advent of Code 2019 Solutions")
        .author("Elijah Voigt")
        .subcommand(SubCommand::with_name("one")
            .about("First day")
            .arg(Arg::with_name("INPUT")
                .help("Input file")
                .required(true)
                .index(1)
            )
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("one") {
        let input = matches.value_of("INPUT").unwrap();
        println!("{}", one::solution(input))
    }
}