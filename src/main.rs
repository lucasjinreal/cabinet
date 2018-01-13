extern crate clap;
extern crate colored;

use clap::{Arg, App, SubCommand, ArgMatches};
use colored::*;


fn parse_args() -> ArgMatches <'static> {
    let matches = App::new("Cabinet")
        .version("1.0.1")
        .about("ultimate tool box, standby at work.")
        .author("Lucas Jin")
        .subcommand(SubCommand::with_name("git")
            .about("Git simplify command, quick push to remote.")
            .version("0.1")
            .author("Lucas Jin")
            .arg(Arg::with_name("comment")
                .help("comment msg of git changes.")
                .index(1)))

        .subcommand(SubCommand::with_name("blog")
            .about("Blog template generate, for my quick writing.")
            .version("0.1")
            .author("Lucas Jin")
            .arg(Arg::with_name("title")
                .help("title of blog")
                .index(1)))

        .subcommand(SubCommand::with_name("code")
            .about("Generate codes project quickly.")
            .version("0.1")
            .author("Lucas Jin")
            .arg(Arg::with_name("name")
                .help("name of project.")
                .index(1)))
        .get_matches();
    return matches;
}


fn main() {
    println!("{} - {}", "Cabinet".yellow().bold(), "The Ultimate Tool Box".blue().bold());
    println!("written by {} with {}.", "Lucas Jin".white().bold(), "Rust".red().bold());
    println!("{} welcome folk and star!", "https://github.com/jinfagang/cabinet".white().italic());

    let matches = parse_args();

    if matches.is_present("git") {
        println!("use {} module", "git".yellow().bold());
    } else if matches.is_present("blog") {
        println!("use {} module", "blog".yellow().bold());
    } else if matches.is_present("code") {
        println!("use {} module", "code".yellow().bold());
    }
}
