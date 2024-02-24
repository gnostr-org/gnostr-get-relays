extern crate getopts;
use getopts::Options;

use std::env;
use std::process;

#[derive(Debug)]
struct NonClone;

extern crate gnostr_get_relays;

// Quick Start
use std::io::{stdout, Write};

use curl::easy::Easy;

// Print a web page onto stdout
fn get_relays_simple() {
    let mut easy = Easy::new();
    easy.url("https://api.nostr.watch/v1/online/").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })
    .unwrap();
    easy.perform().unwrap();

    //println!("{}", easy.response_code().unwrap());
}

fn do_work(inp: &str, _out: Option<String>) {
    //println!("{}", inp);
    let mut input: i32 = inp.trim().parse().expect("Wanted a number");

    if input < 100 {
        input = 100;
    }
    let _output = unsafe { gnostr_get_relays::gnostr_get_relays(input) };
}

pub fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

//pub fn print_usage(program: &str, opts: &Options) {
//    let brief = format!("Usage: {} [options]", program);
//    print!("{}", opts.usage(&brief));
//    process::exit(0);
//}

pub fn print_input(inp: &str, out: Option<String>) {
    println!("{}", inp);
    match out {
        Some(x) => println!("{}", x),
        None => println!("No Output"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    //REF: https://docs.rs/getopts/latest/getopts/struct.Options.html
    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help menu");
    opts.optflag("c", "csv", "return comma seperated list");
    opts.optflag("j", "json", "return json object");
    opts.optopt("o", "output", "set output file name", "NAME");
    opts.optopt("i", "input", "Input an integer (default: 10)", "NUMBER");

    if args.len() >= 1 {
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => {
                println!("Error: {}", f.to_string());
                panic!("{}", f.to_string())
            }
        };
        if matches.opt_present("h") {
            print_usage(&program, &opts);
            process::exit(0);
        }
        if matches.opt_present("j") {
            //returns json
            get_relays_simple();
            process::exit(0);
        }

        let output = matches.opt_str("o");
        //leave input as &Option<String>
        let input = matches.opt_str("i");
        //defed &str
        let value = input.as_deref().unwrap_or("100");
        //returns csv list
        do_work(&value, output);
    }
}
