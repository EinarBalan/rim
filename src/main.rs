//! TEMP
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

mod app;

use std::{env, process};
use app::config::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Invalid arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = app::run(&config) {
        eprintln!("Application encountered error: {}", e);
        process::exit(1);
    }
}