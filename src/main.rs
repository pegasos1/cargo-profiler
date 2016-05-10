#![feature(plugin)]
#![plugin(regex_macros)]

extern crate clap;
extern crate regex;

pub mod profiler;
pub mod parse;
pub mod display;
pub mod err;
pub mod argparse;

use clap::{Arg, App, SubCommand, AppSettings};
use profiler::Profiler;
use parse::callgrind::CallGrindParser;
use parse::cachegrind::CacheGrindParser;
use std::process::Command;
use err::ProfError;
use argparse::{match_profiler, match_binary, parse_num, get_sort_metric};

fn main() {
    real_main();
}

#[cfg(all(unix, target_os = "linux"))]
fn real_main() -> Result<(), ProfError> {
    // create profiler application
    let matches = App::new("cargo-profiler")
                      .bin_name("cargo")
                      .settings(&[AppSettings::SubcommandRequired])
                      .version("1.0")
                      .author("Suchin Gururangan")
                      .about("Profile your binaries")
                      .subcommand(SubCommand::with_name("profiler")
                                      .about("gets callgrind features")
                                      .version("1.0")
                                      .author("Suchin Gururangan")
                                      .subcommand(SubCommand::with_name("callgrind")
                                                      .about("gets callgrind features")
                                                      .version("1.0")
                                                      .author("Suchin Gururangan")
                                                      .arg(Arg::with_name("binary")
                                                               .long("bin")
                                                               .value_name("BINARY")
                                                               .required(true)
                                                               .help("binary you want to \
                                                                      profile"))
                                                      .arg(Arg::with_name("n")
                                                               .short("n")
                                                               .value_name("NUMBER")
                                                               .takes_value(true)
                                                               .help("number of functions you \
                                                                      want")))
                                      .subcommand(SubCommand::with_name("cachegrind")
                                                      .about("gets cachegrind features")
                                                      .version("1.0")
                                                      .author("Suchin Gururangan")
                                                      .arg(Arg::with_name("binary")
                                                               .long("bin")
                                                               .value_name("BINARY")
                                                               .required(true)
                                                               .help("binary you want to \
                                                                      profile"))
                                                      .arg(Arg::with_name("n")
                                                               .short("n")
                                                               .value_name("NUMBER")
                                                               .takes_value(true)
                                                               .help("number of functions you \
                                                                      want"))
                                                      .arg(Arg::with_name("sort")
                                                               .long("sort")
                                                               .value_name("SORT")
                                                               .takes_value(true)
                                                               .help("metric you want to sort \
                                                                      by"))))
                      .get_matches();

    let (m, profiler) = try!(match_profiler(&matches));
    let binary = try!(match_binary(&m));
    let num = try!(parse_num(&m));
    let sort_metric = try!(get_sort_metric(&m));



    // get the name of the binary from the binary argument
    let path = binary.split("/").collect::<Vec<_>>();
    let name = path[path.len() - 1];

    match profiler {
        Profiler::CallGrind { .. } => {
            println!("\nProfiling \x1b[1;36m{} \x1b[0mwith \x1b[1;36mcallgrind...",
                     name)
        }
        Profiler::CacheGrind { .. } => {
            println!("\nProfiling \x1b[1;36m{} \x1b[0mwith \x1b[1;36mcachegrind...",
                     name)
        }
    };

    // get the profiler output
    let output = match profiler {
        Profiler::CallGrind { .. } => try!(profiler.callgrind_cli(&binary)),
        Profiler::CacheGrind { .. } => try!(profiler.cachegrind_cli(&binary)),
    };
    // parse the output into struct
    let parsed = match profiler {
        Profiler::CallGrind { .. } => try!(profiler.callgrind_parse(&output, num)),
        Profiler::CacheGrind { .. } => try!(profiler.cachegrind_parse(&output, num, sort_metric)),
    };

    // pretty-print
    println!("{}", parsed);

    // remove files generated while profiling
    try!(Command::new("rm")
             .arg("cachegrind.out")
             .output());


    try!(Command::new("rm")
             .arg("callgrind.out")
             .output());

    Ok(())
}
