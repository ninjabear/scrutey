#[macro_use]
extern crate serde_json;
extern crate base64;
extern crate regex;
extern crate clap;

mod presenter;
mod strategies;
mod report_card;
mod checker;
mod scrutey;

pub mod families;
pub use self::report_card::ReportCard;
use clap::{Arg, App};
use checker::NaiveChecker;

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const ABOUT: &'static str = "Scrutinize and pretty print formatted data";

use std::io::{self, Read, ErrorKind};
use std::fs::File;
use std::error::Error;

fn get_stdin_util_break() -> Result<String, String> {

    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .map_err(|e| e.description().to_owned() )?;
        
    Ok(buffer)
}

fn read_file_to_string(filename: &str) -> Result<String,String> {

    let mut file = try!(File::open(filename)
                            .map_err(|e| {
                                if e.kind() == ErrorKind::NotFound { 
                                    format!("file '{}' not found", filename)
                                } else if e.kind() == ErrorKind::PermissionDenied {
                                    format!("couldn't read '{}' - permission denied", filename)
                                } else {
                                    e.description().to_owned()
                                }
                            }));

    let mut contents = String::new();

    try!(file.read_to_string(&mut contents)
             .map_err(|e| e.description().to_owned()));

    Ok(contents)
}

fn real_main() -> i32 {
    
    let args = App::new(APP_NAME)
                    .version(VERSION)
                    .about(ABOUT)
                    .arg(Arg::with_name("INPUT")
                            .help("Sets the input file to use. Will use STDIN if no INPUT file is given")
                            .index(1))
                    .get_matches();

    let input = match args.value_of("INPUT") {
        Some(s) => read_file_to_string(s),
        None => get_stdin_util_break()
    };

    let result = scrutey::scrutinize(input, &NaiveChecker::new());

    match result {
        Ok(values) => {
            print!("{}", values);
            EXIT_SUCCESS
        },
        Err(e) => {
            eprint!("{}", e);
            EXIT_FAILURE
        }
    }    
}

fn main() {
    std::process::exit(real_main());
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn read_file_to_string_no_file() {
        let nothing = read_file_to_string(""); 

        match nothing {
            Err(m) => assert_eq!("file '' not found", m),
            _ => unreachable!("bad path")
        }
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn read_file_permission_denied() {
        let nothing = read_file_to_string("tests/resources/unreadable_file"); 

        match nothing {
            Err(m) => assert_eq!("couldn't read 'tests/resources/unreadable_file' - permission denied", m),
            Ok(r) => unreachable!(format!("unexpected - file contains '{}'", r))
        }
    }

    #[test]
    fn good_file_read() {
        let helloworld = read_file_to_string("tests/resources/good_hello_world"); 

        match helloworld {
            Ok(contents) => assert_eq!(include_str!("../tests/resources/good_hello_world"), contents),
            Err(bad) => unreachable!("something went wrong - {}", bad)
        }
    }

}
