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
use io;
use filter;

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
use clap::{App, Arg, ArgMatches, SubCommand};

pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("fpa")
        .version("0.5")
        .author("Pierre Marijon <pierre.marijon@inria.fr>")
        .about("fpa take long read mapping information and filter them")
        .arg(Arg::with_name("input")
             .short("i")
             .long("input")
             .default_value("-")
             .help("Path to input file, use '-' for stdin")
        )
        .arg(Arg::with_name("output")
             .short("o")
             .long("output")
             .default_value("-")
             .help("Path to output file, use '-' for stdout")
        )

        .arg(Arg::with_name("internal-match-threshold")
             .takes_value(true)
             .long("internal-threshold")
             .default_value("0.8")
             .help("A match is internal match if overhang length > match length * internal threshold this option set internal match")
        )
        .arg(Arg::with_name("compression-out")
             .short("z")
             .takes_value(true)
             .long("compression-out")
             .possible_values(&["gzip", "bzip2", "lzma", "no"])
             .help("Output compression format, the input compression format is chosen by default")
        )
        .arg(Arg::with_name("format")
             .short("F")
             .long("format")
             .takes_value(true)
             .help("Force the format used")
             .possible_values(&["paf", "mhap"])
        )
        .subcommand(subcommand::get_keep())
        .subcommand(subcommand::get_drop())
        .subcommand(subcommand::get_rename())
        .subcommand(subcommand::get_index())
        .subcommand(subcommand::get_gfa())
}

pub fn get_subcmd<'a, 'b>(app: &mut App<'a, 'b>) -> std::collections::HashMap<String, ArgMatches<'a>> {
    let basic_cli = vec!["fpa".to_string(), "-i".to_string(), "foo".to_string(), "-o".to_string(), "bar".to_string()];
    let mut sub2matches = std::collections::HashMap::new();

    let mut cli: Vec<String> = std::env::args().collect();
    loop {
        /* parse cli */
        let mut matches = match app.get_matches_from_safe_borrow(cli) {
            Ok(x) => x,
            Err(x) => x.exit(),
        };

        let (name, sub) = match matches.subcommand() {
            (n, Some(s)) => (n, s),
            (n, None) => break,
        };
        
        sub2matches.insert(name.to_string(), sub.clone());

        let (subname, subsub) = match sub.subcommand() {
            (n, Some(s)) => (n, s),
            (n, None) => break,
        };


        if subsub.values_of("").is_none() {
            break;
        }
        
        /* rebuild a new cli*/
        cli = basic_cli.clone();
        cli.push(subname.to_string());
        cli.extend(subsub.values_of("").unwrap().map(|x| x.to_string()));
    }
        
    return sub2matches;
}

pub trait Filters {
    fn pass(&self, r: &io::MappingRecord) -> bool;
    
    fn internal_match(&self) -> f64;

    fn add_filter(&mut self, f: Box<filter::Filter>);
    
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
            self.add_filter(Box::new(filter::Length::new(length_lower.parse::<u64>().unwrap(), std::cmp::Ordering::Less)));
        }
        
        if let Some(length_lower) = m.value_of("length_upper") {
            self.add_filter(Box::new(filter::Length::new(length_lower.parse::<u64>().unwrap(), std::cmp::Ordering::Greater)));
        }
        
        if let Some(name_match) = m.value_of("name_match") {
            self.add_filter(Box::new(filter::NameMatch::new(name_match)));
        }
        
        if m.is_present("same_name") {
            self.add_filter(Box::new(filter::SameName::new()));
        }
    }
}


