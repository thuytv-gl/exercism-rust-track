use anyhow::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt
#[derive(Debug, Default)]
pub struct Flags {
    number: bool,
    name_only: bool,
    insensitive: bool,
    invert: bool,
    match_line: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut ret = Self::default();
        for &f in flags {
            match f {
                "-n" => ret.number = true,
                "-l" => ret.name_only = true,
                "-i" => ret.insensitive = true,
                "-v" => ret.invert = true,
                "-x" => ret.match_line = true,
                _ => panic!("Invalid flag {}", f),
            }
        }
        ret
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut matches = vec![];
    let single = files.len() == 1;
    let pattern = match flags.insensitive {
        true =>  pattern.to_owned().to_lowercase(),
        false =>  pattern.to_owned(),
    };

    for f in files.iter() {
        let file = File::open(f)?;
        let reader = BufReader::new(file);
        for (i, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let check_line = match flags.insensitive {
                true => line.clone().to_lowercase(),
                false => line.clone(),
            };

            let is_match = match flags.match_line {
                true => check_line.eq(&pattern) != flags.invert,
                false => check_line.contains(&pattern) != flags.invert,
            };

            if is_match == false {
                continue;
            }

            matches.push(match flags.name_only {
                true => format!("{0}", f.to_string()),
                false => match (single, flags.number) {
                    (true, true) => format!("{0}:{line}", i+1),
                    (true, false) => line,
                    (false, true) => format!("{0}:{1}:{line}", f.to_string(), i+1),
                    (false, false) => format!("{0}:{line}", f.to_string()),
                }
            });

            if flags.name_only {
                break;
            } 
        }
    }
    Ok(matches)
}

