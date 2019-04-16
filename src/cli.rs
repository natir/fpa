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
use generator;

/* crates use */
use clap::{App, Arg, ArgMatches, SubCommand};

pub fn parser<'a>() -> ArgMatches<'a> {
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
        .subcommand(SubCommand::with_name("keep")
                    .about("fpa keep only mapping match this constraints")
                    .arg(Arg::with_name("containment")
                         .short("c")
                         .long("containment")
                         .help("Keep only containment mapping")
                    )
                    .arg(Arg::with_name("internalmatch")
                         .short("i")
                         .long("internalmatch")
                         .help("Keep only internal mapping")
                    )
                    .arg(Arg::with_name("dovetail")
                         .short("d")
                         .long("dovetail")
                         .help("Keep only dovetail mapping")
                    )
                    .arg(Arg::with_name("length_lower")
                         .short("l")
                         .long("length-lower")
                         .takes_value(true)
                         .help("Keep only mapping with length lower than value")
                    )
                    .arg(Arg::with_name("length_upper")
                         .short("L")
                         .long("length-upper")
                         .takes_value(true)
                         .help("Keep only mapping with length upper than value")
                    )
                    .arg(Arg::with_name("name_match")
                         .short("n")
                         .long("name-match")
                         .takes_value(true)
                         .help("Keep only mapping where one reads match with regex")
                    )
                    .arg(Arg::with_name("same_name")
                         .short("s")
                         .long("same-name")
                         .help("Keep only mapping where reads have same name")
                    )
        )
        .subcommand(SubCommand::with_name("drop")
                    .about("fpa drop mapping match this constraints")
                    .arg(Arg::with_name("containment")
                         .short("c")
                         .long("containment")
                         .help("Drop containment mapping")
                    )
                    .arg(Arg::with_name("internalmatch")
                         .short("i")
                         .long("internalmatch")
                         .help("Drop internal mapping")
                    )
                    .arg(Arg::with_name("dovetail")
                         .short("d")
                         .long("dovetail")
                         .help("Drop dovetail mapping")
                    )
                    .arg(Arg::with_name("length_lower")
                         .short("l")
                         .long("length-lower")
                         .takes_value(true)
                         .help("Drop mapping with length lower than value")
                    )
                    .arg(Arg::with_name("length_upper")
                         .short("L")
                         .long("length-upper")
                         .takes_value(true)
                         .help("Drop mapping with length upper than value")
                    )
                    .arg(Arg::with_name("name_match")
                         .short("n")
                         .long("name-match")
                         .takes_value(true)
                         .help("Drop mapping where one reads match with regex")
                    )
                    .arg(Arg::with_name("same_name")
                         .short("s")
                         .long("same-name")
                         .help("Drop mapping where reads have same name")
                    )
        )
        .subcommand(SubCommand::with_name("rename")
                    .about("fpa rename reads with name you chose or with incremental counter")
                    .arg(Arg::with_name("input")
                         .short("i")
                         .long("input")
                         .takes_value(true)
                         .help("Rename reads with value in path passed as parameter")
                    )
                    .arg(Arg::with_name("output")
                         .short("o")
                         .long("output")
                         .takes_value(true)
                         .help("Write rename table in path passed as parameter")
                    )
        )
        .subcommand(SubCommand::with_name("index")
                    .about("fpa generate a index of mapping passing filter")
                    .arg(Arg::with_name("filename")
                         .short("f")
                         .long("filename")
                         .takes_value(true)
                         .display_order(108)
                         .help("Write index of mapping passing filter in path passed as parameter")
                    )
                    .arg(Arg::with_name("type")
                         .short("t")
                         .long("type")
                         .takes_value(true)
                         .default_value("both")
                         .possible_values(&["query", "target", "both"])
                         .help("Type of index, only reference read when it's query, target or both of them")
                    )
        )
        .subcommand(SubCommand::with_name("gfa")
                    .about("fpa generate a overlap graph in gfa1 format with mapping passing filter")
                    .arg(Arg::with_name("output")
                         .short("o")
                         .long("output")
                         .takes_value(true)
                         .help("Write mapping passing filter in gfa1 graph format in path passed as parameter")
                    )
                    .arg(Arg::with_name("containment")
                         .short("c")
                         .long("containment")
                         .help("Keep containment overlap")
                    )
                    .arg(Arg::with_name("internalmatch")
                         .short("i")
                         .long("internalmatch")
                         .help("Keep internal match overlap")
                    )
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
        .get_matches()
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

pub struct Drop {
    filters: Vec<Box<filter::Filter>>,
    internal_threshold: f64,
}

impl Drop {
    pub fn new(matches: &clap::ArgMatches) -> Self {
        let filters = Vec::new();
        let mut d = Drop {
            filters: filters,
            internal_threshold: matches.value_of("internal-match-threshold").unwrap().parse::<f64>().unwrap(),
        };

        if let Some(drop) = matches.subcommand_matches("drop") {
            d.generate(drop);
        }
        
        return d;
    }
}

impl Filters for Drop {
    fn pass(&self, r: &io::MappingRecord) -> bool {
        return if self.filters.is_empty() {
            true
        } else {
            !self.filters.iter().any(|ref x| x.run(r))
        };
    }
    
    fn internal_match(&self) -> f64 {
        self.internal_threshold
    }
    
    fn add_filter(&mut self, f: Box<filter::Filter>) {
        self.filters.push(f);
    }
}

pub struct Keep {
    filters: Vec<Box<filter::Filter>>,
    internal_threshold: f64,
}

impl Keep {
    pub fn new(matches: &clap::ArgMatches) -> Self {
        let filters = Vec::new();
        let mut k = Keep {
            filters: filters,
            internal_threshold: matches.value_of("internal-match-threshold").unwrap().parse::<f64>().unwrap(),
        };

        if let Some(keep) = matches.subcommand_matches("keep") {
            k.generate(keep);
        }
        
        return k;
    }
}

impl Filters for Keep {
    fn pass(&self, r: &io::MappingRecord) -> bool {
        return if self.filters.is_empty() {
            return true
        } else {
            self.filters.iter().all(|ref x| x.run(r))
        };
    }

    fn internal_match(&self) -> f64 {
        self.internal_threshold
    }
    
    fn add_filter(&mut self, f: Box<filter::Filter>) {
        self.filters.push(f);
    }
}

pub struct Modifier {
    modifiers: Vec<Box<generator::Modifier>>,
}

impl Modifier {
    pub fn new(matches: &clap::ArgMatches) -> Self {
        let mut modifiers: Vec<Box<generator::Modifier>> = Vec::new();

        if let Some(m) = matches.subcommand_matches("rename") {
            if m.is_present("input") {
                modifiers.push(Box::new(generator::Renaming::new(m.value_of("input").unwrap())));
            } else if m.is_present("output") {
                modifiers.push(Box::new(generator::Renaming::new(m.value_of("output").unwrap())));
            }
        }

        let internal_match = matches.value_of("internal-match-threshold").unwrap().parse::<f64>().unwrap();
        if let Some(m) = matches.subcommand_matches("gfa") {
            modifiers.push(
                Box::new(
                    generator::Gfa1::new(
                        m.value_of("output").unwrap().to_string(),
                        m.is_present("internalmatch"),
                        m.is_present("containment"),
                        internal_match
                    )
                )
            )
        }
        
        Modifier {
            modifiers: modifiers,
        }
    }

    pub fn pass(&mut self, r: &mut io::MappingRecord) {
        for mut m in self.modifiers.iter_mut() {
            m.run(r);
        }
    }

    pub fn write(&mut self) {
        for m in self.modifiers.iter_mut() {
            m.write();
        }
    }
}
