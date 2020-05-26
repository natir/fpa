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

/* standard use */
use std::io;
use std::io::{BufReader, BufWriter};

pub fn get_input(input_name: &str) -> (Box<dyn io::Read>, niffler::compression::Format) {
    match input_name {
        "-" => niffler::get_reader(Box::new(BufReader::new(io::stdin())))
            .expect("File is probably empty"),
        _ => niffler::from_path(input_name).expect("File is probably empty"),
    }
}

pub fn choose_compression(
    input_compression: niffler::compression::Format,
    compression_set: bool,
    compression_value: &str,
) -> niffler::compression::Format {
    if !compression_set {
        return input_compression;
    }

    match compression_value {
        "gzip" => niffler::compression::Format::Gzip,
        "bzip2" => niffler::compression::Format::Bzip,
        "lzma" => niffler::compression::Format::Lzma,
        _ => niffler::compression::Format::No,
    }
}

pub fn get_output(output_name: &str, format: niffler::compression::Format) -> Box<dyn io::Write> {
    match output_name {
        "-" => niffler::get_writer(
            Box::new(BufWriter::new(io::stdout())),
            format,
            niffler::compression::Level::One,
        )
        .unwrap(),
        _ => niffler::to_path(output_name, format, niffler::compression::Level::One).unwrap(),
    }
}
