/*
Copyright (c) 2018 Pierre Marijon <pierre.marijon@inria.fr>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
 */

/* project use */
use crate::filter;
use crate::io;

/* local use */
pub mod subcommand;
pub use self::subcommand::*;

pub mod drop;
pub use self::drop::Drop;

pub mod keep;
pub use self::keep::Keep;

pub mod modifier;
pub use self::modifier::*;

/* crates use */
use clap::{App, Arg, ArgMatches};

pub fn app<'a>() -> App<'a> {
    App::new("fpa")
        .version("0.5.1 Sandslash")
        .author("Pierre Marijon <pierre.marijon@inria.fr>")
        .about("fpa take long read mapping information and filter them")
        .arg(Arg::new("input")
             .short('i')
             .long("input")
             .default_value("-")
             .about("Path to input file, use '-' for stdin")
        )
        .arg(Arg::new("output")
             .short('o')
             .long("output")
             .default_value("-")
             .about("Path to output file, use '-' for stdout")
        )

        .arg(Arg::new("internal-match-threshold")
             .takes_value(true)
             .long("internal-threshold")
             .default_value("0.8")
             .about("A match is internal match if overhang length > match length * internal threshold this option set internal match")
        )
        .arg(Arg::new("compression-out")
             .short('z')
             .takes_value(true)
             .long("compression-out")
             .possible_values(&["gzip", "bzip2", "lzma", "no"])
             .about("Output compression format, the input compression format is chosen by default")
        )
        .arg(Arg::new("format")
             .short('F')
             .long("format")
             .takes_value(true)
             .about("Force the format used")
             .possible_values(&["paf", "m4"])
        )
        .subcommand(subcommand::get_keep())
        .subcommand(subcommand::get_drop())
        .subcommand(subcommand::get_rename())
        .subcommand(subcommand::get_index())
        .subcommand(subcommand::get_gfa())
}

pub fn get_subcmd(app: &mut App) -> std::collections::HashMap<String, ArgMatches> {
    let basic_cli = vec![
        "fpa".to_string(),
        "-i".to_string(),
        "foo".to_string(),
        "-o".to_string(),
        "bar".to_string(),
    ];
    let mut sub2matches = std::collections::HashMap::new();

    let mut cli: Vec<String> = std::env::args().collect();
    loop {
        /* parse cli */
        let matches = app
            .try_get_matches_from_mut(cli)
            .unwrap_or_else(|e| e.exit());

        let (name, sub) = match matches.subcommand() {
            Some((n, s)) => (n, s),
            None => break,
        };

        sub2matches.insert(name.to_string(), sub.clone());

        let (subname, subsub) = match sub.subcommand() {
            Some((n, s)) => (n, s),
            None => break,
        };

        if subsub.values_of("").is_none() {
            break;
        }

        /* rebuild a new cli*/
        cli = basic_cli.clone();
        cli.push(subname.to_string());
        cli.extend(subsub.values_of("").unwrap().map(|x| x.to_string()));
    }

    sub2matches
}

pub trait Filters {
    fn pass(&self, r: &dyn io::MappingRecord) -> bool;

    fn internal_match(&self) -> f64;

    fn add_filter(&mut self, f: Box<dyn filter::Filter>);

    fn generate(&mut self, m: &clap::ArgMatches) {
        let internal_match = self.internal_match();
        if m.is_present("containment") {
            self.add_filter(Box::new(filter::Containment::new(internal_match)));
        }

        if m.is_present("internalmatch") {
            self.add_filter(Box::new(filter::InternalMatch::new(internal_match)));
        }

        if m.is_present("dovetail") {
            self.add_filter(Box::new(filter::Dovetails::new(internal_match)));
        }

        if let Some(length_lower) = m.value_of("length_lower") {
            self.add_filter(Box::new(filter::Length::new(
                length_lower.parse::<u64>().unwrap(),
                std::cmp::Ordering::Less,
            )));
        }

        if let Some(length_lower) = m.value_of("length_upper") {
            self.add_filter(Box::new(filter::Length::new(
                length_lower.parse::<u64>().unwrap(),
                std::cmp::Ordering::Greater,
            )));
        }

        if let Some(name_match) = m.value_of("name_match") {
            self.add_filter(Box::new(filter::NameMatch::new(name_match)));
        }

        if m.is_present("same_name") {
            self.add_filter(Box::new(filter::SameName::new()));
        }

        if let Some(sequence_length_lower) = m.value_of("sequence_length_lower") {
            self.add_filter(Box::new(filter::SequenceLength::new(
                sequence_length_lower.parse::<u64>().unwrap(),
                std::cmp::Ordering::Less,
            )));
        }

        if let Some(sequence_length_lower) = m.value_of("sequence_length_upper") {
            self.add_filter(Box::new(filter::SequenceLength::new(
                sequence_length_lower.parse::<u64>().unwrap(),
                std::cmp::Ordering::Greater,
            )));
        }
    }
}
