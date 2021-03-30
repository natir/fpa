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
use clap::{App, Arg};

pub fn get_keep<'a>() -> App<'a> {
    App::new("keep")
        .setting(clap::AppSettings::AllowExternalSubcommands)
        .about("fpa keep only mapping match this constraints")
        .arg(
            Arg::new("containment")
                .short('c')
                .long("containment")
                .about("Keep only containment mapping"),
        )
        .arg(
            Arg::new("internalmatch")
                .short('i')
                .long("internalmatch")
                .about("Keep only internal mapping"),
        )
        .arg(
            Arg::new("dovetail")
                .short('d')
                .long("dovetail")
                .about("Keep only dovetail mapping"),
        )
        .arg(
            Arg::new("length_lower")
                .short('l')
                .long("length-lower")
                .takes_value(true)
                .about("Keep only mapping with length lower than value"),
        )
        .arg(
            Arg::new("length_upper")
                .short('L')
                .long("length-upper")
                .takes_value(true)
                .about("Keep only mapping with length upper than value"),
        )
        .arg(
            Arg::new("name_match")
                .short('n')
                .long("name-match")
                .takes_value(true)
                .about("Keep only mapping where one reads match with regex"),
        )
        .arg(
            Arg::new("same_name")
                .short('m')
                .long("same-name")
                .about("Keep only mapping where reads have same name"),
        )
        .arg(
            Arg::new("sequence_length_lower")
                .short('s')
                .long("sequence-length-lower")
                .takes_value(true)
                .about("Keep only mapping where one reads have length lower than value"),
        )
        .arg(
            Arg::new("sequence_length_upper")
                .short('S')
                .long("sequence-length-upper")
                .takes_value(true)
                .about("Keep only mapping where one reads have length upper than value"),
        )
}

pub fn get_drop<'a>() -> clap::App<'a> {
    App::new("drop")
        .setting(clap::AppSettings::AllowExternalSubcommands)
        .about("fpa drop mapping match this constraints")
        .arg(
            Arg::new("containment")
                .short('c')
                .long("containment")
                .about("Drop containment mapping"),
        )
        .arg(
            Arg::new("internalmatch")
                .short('i')
                .long("internalmatch")
                .about("Drop internal mapping"),
        )
        .arg(
            Arg::new("dovetail")
                .short('d')
                .long("dovetail")
                .about("Drop dovetail mapping"),
        )
        .arg(
            Arg::new("length_lower")
                .short('l')
                .long("length-lower")
                .takes_value(true)
                .about("Drop mapping with length lower than value"),
        )
        .arg(
            Arg::new("length_upper")
                .short('L')
                .long("length-upper")
                .takes_value(true)
                .about("Drop mapping with length upper than value"),
        )
        .arg(
            Arg::new("name_match")
                .short('n')
                .long("name-match")
                .takes_value(true)
                .about("Drop mapping where one reads match with regex"),
        )
        .arg(
            Arg::new("same_name")
                .short('m')
                .long("same-name")
                .about("Drop mapping where reads have same name"),
        )
        .arg(
            Arg::new("sequence_length_lower")
                .short('s')
                .long("sequence-length-lower")
                .takes_value(true)
                .about("Drop mapping where one reads have length lower than value"),
        )
        .arg(
            Arg::new("sequence_length_upper")
                .short('S')
                .long("sequence-length-upper")
                .takes_value(true)
                .about("Drop mapping where one reads have length upper than value"),
        )
}

pub fn get_rename<'a>() -> clap::App<'a> {
    App::new("rename")
        .setting(clap::AppSettings::AllowExternalSubcommands)
        .about("fpa rename reads with name you chose or with incremental counter")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .takes_value(true)
                .about("Rename reads with value in path passed as parameter"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .takes_value(true)
                .about("Write rename table in path passed as parameter"),
        )
}

pub fn get_index<'a>() -> clap::App<'a> {
    App::new("index")
        .setting(clap::AppSettings::AllowExternalSubcommands)
        .about("fpa generate a index of mapping passing filter")
        .arg(
            Arg::new("filename")
                .short('f')
                .long("filename")
                .takes_value(true)
                .display_order(108)
                .about("Write index of mapping passing filter in path passed as parameter"),
        )
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .takes_value(true)
                .default_value("both")
                .possible_values(&["query", "target", "both"])
                .about(
                    "Type of index, only reference read when it's query, target or both of them",
                ),
        )
}

pub fn get_gfa<'a>() -> clap::App<'a> {
    App::new("gfa")
        .setting(clap::AppSettings::AllowExternalSubcommands)
        .about("fpa generate a overlap graph in gfa1 format with mapping passing filter")
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .required(true)
                .takes_value(true)
                .about(
                    "Write mapping passing filter in gfa1 graph format in path passed as parameter",
                ),
        )
        .arg(
            Arg::new("containment")
                .short('c')
                .long("containment")
                .about("Keep containment overlap"),
        )
        .arg(
            Arg::new("internalmatch")
                .short('i')
                .long("internalmatch")
                .about("Keep internal match overlap"),
        )
}
