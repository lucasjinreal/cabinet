extern crate clap;
extern crate colored;


use clap::{Arg, App, SubCommand, ArgMatches};
use colored::*;
use std::{process};

mod blog;
mod templates;


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
                .index(1))
            .arg(Arg::with_name("date")
                .short("d")
                .help("using date as prefix or not.")))

        .subcommand(SubCommand::with_name("code")
            .about("Generate codes project quickly.")
            .version("0.1")
            .author("Lucas Jin")
            .arg(Arg::with_name("name")
                .help("name of project.")
                .index(1)))

        .subcommand(SubCommand::with_name("template")
            .about("Quickly show some template text: Makefile, CMakeLists.txt etc.")
            .version("0.1")
            .author("Lucas Jin")
            .arg(Arg::with_name("name")
                .short("n")
                .help("Template name {make, makefile, cmake, etc...}")
                .index(1)))

        .get_matches();
    return matches;
}


fn main() {
    println!("{} - {}", "Cabinet".yellow().bold(), "The Ultimate Tool Box".yellow().bold());
    println!("written by {} with {}.", "Lucas Jin".green().bold(), "Rust".red().bold());
    println!("{} \nwelcome folk and star!", "https://github.com/jinfagang/cabinet");

    let matches = parse_args();

    if matches.is_present("git") {
        println!("using {} module", "git".yellow().bold());
        println!("we are not support git anymore, removed from cabinet.");

    } else if matches.is_present("blog") {
        println!("using {} module", "blog".yellow().bold());

        let mut _title = "no_title";
        let mut is_date = false;
        if let Some(blog_matches) = matches.subcommand_matches("blog") {
            if blog_matches.is_present("date") {
                is_date = true;
            }

            if blog_matches.is_present("title") {
                _title = blog_matches.value_of("title").unwrap();
            } else {
                println!("no title provided.");
                process::exit(1);
            }

            blog::generate_blog_template(is_date, _title);
        }


    } else if matches.is_present("code") {
        println!("using {} module", "code".yellow().bold());

    } else if matches.is_present("template") {
        println!("using {} module", "template".yellow().bold());

        if let Some(template_matches) = matches.subcommand_matches("template") {
            let mut temp_name;
            if template_matches.is_present("name") {
                // print makefile template
                temp_name = template_matches.value_of("name").unwrap();
                let mut temp;
                temp = templates::get_template_by_name(temp_name);
                println!("{}", temp);

            }
        }
    }
}
