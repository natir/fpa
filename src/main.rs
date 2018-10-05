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

extern crate csv;
extern crate clap;
extern crate regex;
extern crate serde;
extern crate bzip2;
extern crate flate2;
extern crate xz2;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate enum_primitive;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

/* project mod */
mod io;
mod file;
mod filter;

/* crates use */
use clap::{App, Arg, ArgMatches};

/* std use */

/* project use */
#[allow(unused_imports)] 
use filter::Filter;

fn main() {

    let matches = App::new("fpaf")
        .version("0.1 Mewto")
        .author("Pierre Marijon <pierre.marijon@inria.fr>")
        .about("fpaf take in stdin PAF information and write in stdout the valid")
        .arg(Arg::with_name("delete_containment")
             .short("c")
             .display_order(10)
             .long("delete-containment")
             .help("If match are containment match is discard")
             )
        .arg(Arg::with_name("keep_containment")
             .short("C")
             .display_order(20)
             .long("keep-containment")
             .help("Only containment match is keeped")
             )
        .arg(Arg::with_name("delete_internalmatch")
             .short("i")
             .display_order(30)
             .long("delete-internalmatch")
             .help("If match are an internal match is discard")
             )
        .arg(Arg::with_name("keep_internalmatch")
             .short("I")
             .display_order(40)
             .long("keep-internalmatch")
             .help("Only internal match overlap is keeped")
             )
        .arg(Arg::with_name("delete_dovetail")
             .short("d")
             .display_order(42)
             .long("delete-dovetail")
             .help("If match are an dovetail is discard")
             )
        .arg(Arg::with_name("keep_dovetail")
             .short("D")
             .display_order(48)
             .long("keep-dovetail")
             .help("Only dovetail overlap is keeped")
             )
        .arg(Arg::with_name("delete_length_lower")
             .short("l")
             .display_order(50)
             .takes_value(true)
             .long("delete-lower")
             .help("If match length is lower than the value is discard")
             )
        .arg(Arg::with_name("delete_length_greater")
             .short("L")
             .display_order(60)
             .takes_value(true)
             .long("delete-upper")
             .help("If match length is upper than the value is discard")
             )
        .arg(Arg::with_name("delete_name_match")
             .short("m")
             .display_order(70)
             .takes_value(true)
             .long("delete-match")
             .help("If match contain read name match with regex is discard")
             )
        .arg(Arg::with_name("keep_name_match")
             .short("M")
             .display_order(80)
             .takes_value(true)
             .long("keep-match")
             .help("Only match contain read name match with regex keeped")
             )
        .arg(Arg::with_name("delete_samename")
             .short("s")
             .display_order(90)
             .takes_value(true)
             .long("delete-same-name")
             .help("If self match is discard")
             )
        .arg(Arg::with_name("keep_samename")
             .short("S")
             .display_order(100)
             .takes_value(true)
             .long("keep-same-name")
             .help("Only self match are keeped")
             )
        .arg(Arg::with_name("internal-match-threshold")
             .display_order(105)
             .takes_value(true)
             .long("internal-threshold")
             .default_value("0.8")
             .help("A match is internal match if overhang length > match length * internal threshold this option set internal match")
             )
        .arg(Arg::with_name("compression-out")
             .display_order(110)
             .takes_value(true)
             .long("compression-out")
             .possible_values(&["gzip", "bzip2", "lzma", "no"])
             .help("Output compression format, the input compression format is chosen by default")
             )
        .arg(Arg::with_name("format")
             .short("F")
             .long("format")
             .display_order(120)
             .takes_value(true)
             .help("Force the format used")
             .possible_values(&["paf", "mhap"])
             )
        .arg(Arg::with_name("output")
             .takes_value(true)
             .default_value("-")
             )
        .arg(Arg::with_name("input")
             .multiple(true)
             .takes_value(true)
             .default_value("-")
             )
        .get_matches();

    /* Manage input and output file */
    let mut compression: file::CompressionFormat = file::CompressionFormat::No;
    let mut inputs: Vec<Box<std::io::Read>> = Vec::new();
    
    let formats = if matches.is_present("format") {
        matches.value_of("format").unwrap()
    } else {
        "paf"
    };

    for input_name in matches.values_of("input").unwrap() {
        let tmp = file::get_input(input_name);
        inputs.push(tmp.0);
        compression = tmp.1;
    }

    let out_compression = file::choose_compression(
        compression,
        matches.is_present("compression-out"),
        matches.value_of("compression-out").unwrap_or("no"),
    );

    let output: Box<std::io::Write> =
        file::get_output(matches.value_of("output").unwrap(), out_compression);

    let mut writer = io::paf::Writer::new(output);

    /* Manage filter */
    let filters = generate_filters(&matches);

    /* Do the job */
    for input in inputs {
        let mut reader = if formats == "paf" {
            io::paf::Reader::new(input)
        } else {
            io::paf::Reader::new(input)
        };

        for result in reader.records() {
            let record = result.expect("Trouble during read of input");

            if filters.iter().any(|ref x| x.run(&record)) {
                continue;
            }

            writer.write(&record).expect("Trouble during write of output");
        }
    }
}

fn generate_filters(matches: &ArgMatches) -> Vec<Box<filter::Filter>> {
    let mut filters : Vec<Box<filter::Filter>> = Vec::new();
    
    if matches.is_present("delete_internalmatch") {
        filters.push(Box::new(filter::InternalMatch::new(false)));
    }
    
    if matches.is_present("keep_internalmatch") {
        filters.push(Box::new(filter::InternalMatch::new(true)));
    }
    
    if matches.is_present("delete_containment") {
        filters.push(Box::new(filter::Containment::new(false)));
    }
    
    if matches.is_present("keep_containment") {
        filters.push(Box::new(filter::Containment::new(true)));
    }
    
    if matches.is_present("delete_dovetail") {
        filters.push(Box::new(filter::Dovetails::new(false)));
    }
    
    if matches.is_present("keep_dovetail") {
        filters.push(Box::new(filter::Dovetails::new(true)));
    }
    
    if matches.is_present("delete_length_lower") {
        filters.push(
            Box::new(
                filter::Length::new(
                    matches.value_of("delete_length_lower").unwrap().parse::<u64>().unwrap(),
                    std::cmp::Ordering::Less
                )
            )
        );
    }
    
    if matches.is_present("delete_length_greater") {
        filters.push(
            Box::new(
                filter::Length::new(
                    matches.value_of("delete_length_greater").unwrap().parse::<u64>().unwrap(),
                    std::cmp::Ordering::Greater
                )
            )
        );
    }
    
    if matches.is_present("delete_name_match") {
        filters.push(
            Box::new(
                filter::NameMatch::new(
                    matches.value_of("delete_name_match").unwrap(),
                    false
                )
            )
        );
    }
    
    if matches.is_present("keep_name_match") {
        filters.push(
            Box::new(
                filter::NameMatch::new(
                    matches.value_of("keep_name_match").unwrap(),
                    true
                )
            )
        );
    }
    
    if matches.is_present("delete_samename") {
        filters.push(Box::new(filter::SameName::new(false)));
    }
    
    if matches.is_present("keep_samename") {
        filters.push(Box::new(filter::SameName::new(true)));
    }

	return filters;
}
