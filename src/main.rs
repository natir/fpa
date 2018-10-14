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
mod cli;
mod file;
mod work;
mod filter;
mod modifier;

fn main() {

    let matches = cli::parser();

    /* Manage input and output file */
    let mut compression: file::CompressionFormat = file::CompressionFormat::No;
    let mut inputs: Vec<Box<std::io::Read>> = Vec::new();
    
    let format = if matches.is_present("format") {
        match matches.value_of("format").unwrap() {
            "paf" => work::InOutFormat::Paf,
            "mhap" => work::InOutFormat::Mhap,
            _ => work::InOutFormat::Paf,
        }
    } else {
        work::InOutFormat::Paf
    };
    
    let mode = if matches.is_present("mode") {
        match matches.value_of("mode").unwrap() {
            "basic" => work::Mode::Basic,
            "gfa1" => work::Mode::Gfa1,
            //"gfa2" => work::Mode::Gfa2, // Not yet support
            _ => work::Mode::Basic,
        }
    } else {
        work::Mode::Basic
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

    let mut output: Box<std::io::Write> =
        file::get_output(matches.value_of("output").unwrap(), out_compression);

    /* Manage filter */
    let filters = cli::generate_filters(&matches);

    /* Manage modifier */
    let mut modifiers = cli::generate_modifiers(&matches);

    work::run(inputs, &mut output, &filters, &mut modifiers, &matches, format, mode);
}
