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

/* crates use */
use clap::{App, Arg, SubCommand};

pub fn get_keep<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("keep")
        .setting(clap::AppSettings::AllowExternalSubcommands)
        .about("fpa keep only mapping match this constraints")
        .arg(
            Arg::with_name("containment")
                .short("c")
                .long("containment")
                .help("Keep only containment mapping"),
        )
        .arg(
            Arg::with_name("internalmatch")
                .short("i")
                .long("internalmatch")
                .help("Keep only internal mapping"),
        )
        .arg(
            Arg::with_name("dovetail")
                .short("d")
                .long("dovetail")
                .help("Keep only dovetail mapping"),
        )
        .arg(
            Arg::with_name("length_lower")
                .short("l")
                .long("length-lower")
                .takes_value(true)
                .help("Keep only mapping with length lower than value"),
        )
        .arg(
            Arg::with_name("length_upper")
                .short("L")
                .long("length-upper")
                .takes_value(true)
                .help("Keep only mapping with length upper than value"),
        )
        .arg(
            Arg::with_name("name_match")
                .short("n")
                .long("name-match")
                .takes_value(true)
                .help("Keep only mapping where one reads match with regex"),
        )
        .arg(
            Arg::with_name("same_name")
                .short("m")
                .long("same-name")
                .help("Keep only mapping where reads have same name"),
        )
        .arg(
            Arg::with_name("sequence_length_lower")
                .short("s")
                .long("sequence-length-lower")
                .takes_value(true)
                .help("Keep only mapping where one reads have length lower than value"),
        )
        .arg(
            Arg::with_name("sequence_length_upper")
                .short("S")
                .long("sequence-length-upper")
                .takes_value(true)
                .help("Keep only mapping where one reads have length upper than value"),
        )
}

pub fn get_drop<'a, 'b>() -> clap::App<'a, 'b> {
    SubCommand::with_name("drop")
        .setting(clap::AppSettings::AllowExternalSubcommands)
        .about("fpa drop mapping match this constraints")
        .arg(
            Arg::with_name("containment")
                .short("c")
                .long("containment")
                .help("Drop containment mapping"),
        )
        .arg(
            Arg::with_name("internalmatch")
                .short("i")
                .long("internalmatch")
                .help("Drop internal mapping"),
        )
        .arg(
            Arg::with_name("dovetail")
                .short("d")
                .long("dovetail")
                .help("Drop dovetail mapping"),
        )
        .arg(
            Arg::with_name("length_lower")
                .short("l")
                .long("length-lower")
                .takes_value(true)
                .help("Drop mapping with length lower than value"),
        )
        .arg(
            Arg::with_name("length_upper")
                .short("L")
                .long("length-upper")
                .takes_value(true)
                .help("Drop mapping with length upper than value"),
        )
        .arg(
            Arg::with_name("name_match")
                .short("n")
                .long("name-match")
                .takes_value(true)
                .help("Drop mapping where one reads match with regex"),
        )
        .arg(
            Arg::with_name("same_name")
                .short("m")
                .long("same-name")
                .help("Drop mapping where reads have same name"),
        )
        .arg(
            Arg::with_name("sequence_length_lower")
                .short("s")
                .long("sequence-length-lower")
                .takes_value(true)
                .help("Drop mapping where one reads have length lower than value"),
        )
        .arg(
            Arg::with_name("sequence_length_upper")
                .short("S")
                .long("sequence-length-upper")
                .takes_value(true)
                .help("Drop mapping where one reads have length upper than value"),
        )
}

pub fn get_rename<'a, 'b>() -> clap::App<'a, 'b> {
    SubCommand::with_name("rename")
        .setting(clap::AppSettings::AllowExternalSubcommands)
        .about("fpa rename reads with name you chose or with incremental counter")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .takes_value(true)
                .help("Rename reads with value in path passed as parameter"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("Write rename table in path passed as parameter"),
        )
}

pub fn get_index<'a, 'b>() -> clap::App<'a, 'b> {
    SubCommand::with_name("index")
        .setting(clap::AppSettings::AllowExternalSubcommands)
        .about("fpa generate a index of mapping passing filter")
        .arg(
            Arg::with_name("filename")
                .short("f")
                .long("filename")
                .takes_value(true)
                .display_order(108)
                .help("Write index of mapping passing filter in path passed as parameter"),
        )
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("type")
                .takes_value(true)
                .default_value("both")
                .possible_values(&["query", "target", "both"])
                .help("Type of index, only reference read when it's query, target or both of them"),
        )
}

pub fn get_gfa<'a, 'b>() -> clap::App<'a, 'b> {
    SubCommand::with_name("gfa")
        .setting(clap::AppSettings::AllowExternalSubcommands)
        .about("fpa generate a overlap graph in gfa1 format with mapping passing filter")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
		.required(true)
                .takes_value(true)
                .help(
                    "Write mapping passing filter in gfa1 graph format in path passed as parameter",
                ),
        )
        .arg(
            Arg::with_name("containment")
                .short("c")
                .long("containment")
                .help("Keep containment overlap"),
        )
        .arg(
            Arg::with_name("internalmatch")
                .short("i")
                .long("internalmatch")
                .help("Keep internal match overlap"),
        )
}
