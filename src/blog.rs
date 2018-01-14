extern crate colored;
extern crate time;
extern crate chrono;


use colored::*;
use chrono::prelude::*;


pub fn generate_blog_template(use_date: bool, title: &str) {
    if use_date {
        println!("\nuse date as title prefix {}", "ON".yellow().bold());

        let now = Utc::now();
        println!("{}", now.format("%b %-d, %-I:%M").to_string());
        println!("generate blog with name: {}", title)
    } else {
        println!("\nuse date as title prefix {}", "OFF".yellow().bold());
        println!("generate blog with name: {}", title)
    }
}