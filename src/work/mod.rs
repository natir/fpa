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
use generator;

/* std use */
use std;
use std::rc::Rc;
use std::cell::RefCell;

/* crate use */
use clap::ArgMatches;

/* module declaration */
mod gfa;
mod basic;

#[derive(Debug, PartialEq)]
pub enum InOutFormat {
    Paf,
    Mhap,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Basic,
    Gfa1,
    // Gfa2, // not support yet
}

pub fn run<R: std::io::Read, W: std::io::Write>(
    inputs: Vec<R>,
    output: &mut W,
    filters: &Vec<Box<filter::Filter>>,
    modifiers: &mut Vec<Rc<RefCell<modifier::Modifier>>>,
    matches: &ArgMatches,
    format: InOutFormat,
    mode: Mode,
) {

    if mode == Mode::Basic {
        basic::basic(inputs, output, filters, modifiers, format);
    } else {
        gfa::gfa1(inputs, output, modifiers, matches, format);
    }
}
