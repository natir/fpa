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
use filter;
use modifier;

/* std use */
use std;
use std::rc::Rc;
use std::cell::RefCell;

/* crates use */
use clap::{App, Arg, ArgMatches};

pub fn parser<'a>() -> ArgMatches<'a> {
    App::new("fpa")
        .version("0.3 Meowth")
        .author("Pierre Marijon <pierre.marijon@inria.fr>")
        .about("fpa take long read mapping information and filter them")
        .arg(Arg::with_name("delete_containment")
             .short("c")
             .display_order(10)
             .long("delete-containment")
             .help("If match are containment match is discard, in gfa1 mode containment is not included")
             )
        .arg(Arg::with_name("keep_containment")
             .short("C")
             .display_order(20)
             .long("keep-containment")
             .help("Only containment match is keeped, in gfa1 mode containment is included")
             )
        .arg(Arg::with_name("delete_internalmatch")
             .short("i")
             .display_order(30)
             .long("delete-internalmatch")
             .help("If match are an internal match is discard, in gfa1 mode internalmatch is not included")
             )
        .arg(Arg::with_name("keep_internalmatch")
             .short("I")
             .display_order(40)
             .long("keep-internalmatch")
             .help("Only internal match overlap is keeped, in gfa1 mode internalmatch is included")
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
             .long("delete-same-name")
             .help("If self match is discard")
             )
        .arg(Arg::with_name("keep_samename")
             .short("S")
             .display_order(100)
             .long("keep-same-name")
             .help("Only self match are keeped")
             )
        .arg(Arg::with_name("modifier-renaming")
             .short("r")
             .long("rename")
             .takes_value(true)
             .display_order(107)
             .help("Rename read with value in file passed as parameter if exist or by index store in file passed as parameter are empty")
             )
        .arg(Arg::with_name("internal-match-threshold")
             .takes_value(true)
             .display_order(105)
             .long("internal-threshold")
             .default_value("0.8")
             .help("A match is internal match if overhang length > match length * internal threshold this option set internal match")
             )
        .arg(Arg::with_name("compression-out")
             .short("z")
             .takes_value(true)
             .display_order(110)
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
        .arg(Arg::with_name("mode")
             .short("o")
             .long("output-mode")
             .display_order(120)
             .takes_value(true)
             .default_value("basic")
             .help("basic: output in same format as input, gfa1: output is in gfa1 overlap graph format (by default flag -I and -C are up)")
             .possible_values(&["basic", "gfa1"])
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
        .get_matches()
}


pub fn generate_modifiers<'a>(matches: &ArgMatches) -> Vec<Rc<RefCell<modifier::Modifier>>> {
    let mut modifiers: Vec<Rc<RefCell<modifier::Modifier>>> = Vec::new();

    if matches.is_present("modifier-renaming") {
        let rename_file = matches.value_of("modifier-renaming").unwrap();
        modifiers.push(Rc::new(RefCell::new(modifier::Renaming::new(rename_file))));
    }

    return modifiers;
}


pub fn generate_filters(matches: &ArgMatches) -> Vec<Box<filter::Filter>> {

    let mut filters: Vec<Box<filter::Filter>> = Vec::new();

    generate_type_filters(matches, &mut filters);
    generate_other_filters(matches, &mut filters);

    return filters;
}

pub fn generate_type_filters(matches: &ArgMatches, filters: &mut Vec<Box<filter::Filter>>) {

    let internal_match_t = matches
        .value_of("internal-match-threshold")
        .unwrap()
        .parse::<f64>()
        .unwrap();

    if matches.is_present("delete_internalmatch") {
        filters.push(Box::new(
            filter::InternalMatch::new(internal_match_t, false),
        ));
    }

    if matches.is_present("keep_internalmatch") {
        filters.push(Box::new(filter::InternalMatch::new(internal_match_t, true)));
    }

    if matches.is_present("delete_containment") {
        filters.push(Box::new(filter::Containment::new(internal_match_t, false)));
    }

    if matches.is_present("keep_containment") {
        filters.push(Box::new(filter::Containment::new(internal_match_t, true)));
    }

    if matches.is_present("delete_dovetail") {
        filters.push(Box::new(filter::Dovetails::new(internal_match_t, false)));
    }

    if matches.is_present("keep_dovetail") {
        filters.push(Box::new(filter::Dovetails::new(internal_match_t, true)));
    }
}

pub fn generate_other_filters(matches: &ArgMatches, filters: &mut Vec<Box<filter::Filter>>) {
    if matches.is_present("delete_length_lower") {
        filters.push(Box::new(filter::Length::new(
            matches
                .value_of("delete_length_lower")
                .unwrap()
                .parse::<u64>()
                .unwrap(),
            std::cmp::Ordering::Less,
        )));
    }

    if matches.is_present("delete_length_greater") {
        filters.push(Box::new(filter::Length::new(
            matches
                .value_of("delete_length_greater")
                .unwrap()
                .parse::<u64>()
                .unwrap(),
            std::cmp::Ordering::Greater,
        )));
    }

    if matches.is_present("delete_name_match") {
        filters.push(Box::new(filter::NameMatch::new(
            matches.value_of("delete_name_match").unwrap(),
            false,
        )));
    }

    if matches.is_present("keep_name_match") {
        filters.push(Box::new(filter::NameMatch::new(
            matches.value_of("keep_name_match").unwrap(),
            true,
        )));
    }

    if matches.is_present("delete_samename") {
        filters.push(Box::new(filter::SameName::new(false)));
    }

    if matches.is_present("keep_samename") {
        filters.push(Box::new(filter::SameName::new(true)));
    }
}
